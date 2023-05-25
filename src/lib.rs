use darling::{
    self,
    ast::Data,
    export::syn::{parse_macro_input, parse_str, DeriveInput, Expr, Ident},
    FromDeriveInput, FromField,
};
use heck::ToSnakeCase;
use proc_macro::TokenStream;

use quote::{quote, ToTokens};

#[proc_macro_derive(EguiFormer, attributes(ef))]
pub fn ef(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let former = EguiFormer::from_derive_input(&input).unwrap();
    former.to_token_stream().into()
}

#[derive(FromDeriveInput, Debug)]
struct EguiFormer {
    ident: Ident,
    data: Data<(), EguiFormerField>,
}

impl EguiFormer {
    fn grid_name(&self) -> String {
        (self.ident.to_string() + "_former").to_snake_case()
    }
}
impl ToTokens for EguiFormer {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let input_ident = self.ident.clone();
        let grid_name = self.grid_name();
        let fields = match &self.data {
            Data::Enum(_e) => todo!(),
            Data::Struct(s) => s
                .iter()
                .map(|f| {
                    let label = f.field_string();
                    let mut func = None;
                    macro_rules! repeat_ui_impl {
                            ($( $case:ident ),*) => {{
                                $(
                                if func.is_none() && f.$case {
                                    func = Some(f.$case());
                                }

                            )*
                            }};
                        }

                    repeat_ui_impl!(
                        drag_angle,
                        combobox,
                        hyperlink,
                        text,
                        text_multi,
                        color_edit,
                        drag,
                        button,
                        small_button,
                        toggle_value,
                        radio,
                        radio_value,
                        heading,
                        monospace
                    );

                    if func.is_none() && f.customize.is_some() {
                        func = Some(f.customize.clone().unwrap())
                    }
                    if func.is_none() {
                        func = Some("ui.text_edit_singleline(&mut self.{})".to_owned())
                    }

                    let func = func.unwrap().as_str().replace("{}", &label);
                    let func = parse_str::<Expr>(&func).unwrap();
                    quote!(
                            ui.label(#label);
                            #func;
                            ui.end_row();
                    )
                })
                .collect::<Vec<proc_macro2::TokenStream>>(),
        };

        let output = quote!(
            impl EguiFormer for #input_ident {
                fn former_ui(&mut self, ui: &mut Ui) {
                    egui::Grid::new(#grid_name)
                        .num_columns(2)
                        .spacing([20.0, 10.0])
                        .striped(true)
                        .show(ui, |ui| {
                            #(#fields)*
                        });
                }
            }
        );
        tokens.extend(output);
    }
}
#[derive(FromField, Default, Debug)]
#[darling(default, attributes(ef))]
struct EguiFormerField {
    ident: Option<Ident>,
    customize: Option<String>,
    drag_angle: bool,
    combobox: bool,
    hyperlink: bool,
    text: bool,
    text_multi: bool,
    color_edit: bool,
    drag: bool,
    button: bool,
    small_button: bool,
    toggle_value: bool,
    radio: bool,
    radio_value: bool,
    heading: bool,
    monospace: bool,
}
// #[derive(FromMeta)]
// struct EguiFormerFieldMeta {}
impl EguiFormerField {
    fn drag_angle(&self) -> String {
        "ui.drag_angle(&mut self.{})".to_string()
    }

    fn combobox(&self) -> String {
        "ui.checkbox(&mut self.{}, \"\")".to_string()
    }
    fn drag(&self) -> String {
        "ui.add(egui::DragValue::new(&mut self.{}))".to_string()
    }
    fn hyperlink(&self) -> String {
        "ui.hyperlink(&mut self.{})".to_string()
    }
    fn text(&self) -> String {
        "ui.text_edit_singleline(&mut self.{})".to_string()
    }
    fn text_multi(&self) -> String {
        "ui.text_edit_multiline(&mut self.{})".to_string()
    }
    fn color_edit(&self) -> String {
        // "ui.color_edit(&mut self.{})".to_string()
        "ui.color_edit_button_srgba(&mut self.{})".to_string()
    }
    fn button(&self) -> String {
        "ui.button(&self.{})".to_string()
    }
    fn small_button(&self) -> String {
        "ui.small_button(&mut self.{})".to_string()
    }
    fn toggle_value(&self) -> String {
        "ui.toggle_value(&mut self.{}.0,&self.{}.1)".to_string()
    }
    fn radio(&self) -> String {
        "ui.radio(&mut self.{})".to_string()
    }
    fn radio_value(&self) -> String {
        "ui.radio_value(&mut self.{}.0,&self.{}.1,&self.{}.2)".to_string()
    }
    fn heading(&self) -> String {
        "ui.heading(&mut self.{})".to_string()
    }
    fn monospace(&self) -> String {
        "ui.monospace(&mut self.{})".to_string()
    }
}
trait OptionExt<T> {
    fn ref_unwrap(&self) -> &T;
}
impl<T> OptionExt<T> for Option<T> {
    fn ref_unwrap(&self) -> &T {
        self.as_ref().unwrap()
    }
}

impl EguiFormerField {
    fn field(&self) -> &Ident {
        self.ident.ref_unwrap()
    }
    fn field_string(&self) -> String {
        self.field().to_string()
    }
}
