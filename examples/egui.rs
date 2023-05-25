use egui::{Color32, Ui};
use egui_former_macro::EguiFormer;

pub trait EguiFormer {
    fn former_ui(&mut self, ui: &mut Ui);
}
#[allow(unused_must_use)]
#[derive(EguiFormer)]
struct Command {
    // #[ef(customize = "1")]
    // #[ef]
    executable: String,
    #[ef(drag_angle)]
    p2: f32,
    #[ef(combobox)]
    p3: bool,
    #[ef(hyperlink)]
    p4: String,
    #[ef(text)]
    p5: String,
    #[ef(text_multi)]
    p6: String,
    #[ef(color_edit)]
    p7: Color32,
    #[ef(drag)]
    p8: f32,
    #[ef(button)]
    p9: String,
}

// impl EguiFormer for Command {
//     fn former_ui(&mut self, ui: &mut Ui) {
//         egui::Grid::new("command_former")
//             .num_columns(2)
//             .spacing([20.0, 10.0])
//             .striped(true)
//             .show(ui, |ui| {
//                 ui.label("executable");
//                 ui.end_row();
//             });
//     }
// }
fn main() {
    // let input = quote! {
    //     #[egui_former]
    //     pub struct AngleKnobPage {
    //         value: f32,
    //     }
    // };

    // let egui_former: DeriveInput = parse_str(input).unwrap();
    // let output = former(quote!().into(), input);
    // eprintln!("{}", output);
    // let receiver = MyInputReceiver::from_derive_input(&parsed).unwrap();
    // let tokens = quote!(#receiver);

    // println!(
    //     r#"
    // INPUT:

    // {}

    // PARSED AS:

    // {:?}

    // EMITS:

    // {}
    //     "#,
    //     input, receiver, tokens
    // );
}
