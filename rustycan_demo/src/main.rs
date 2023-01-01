use macroquad::prelude::*;
use rustycan_macro::rustycan_ui;

#[macroquad::main("Rustycan with egui and macroquad")]
async fn main() {
    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Rustycan 🤗 egui ❤ macroquad").show(egui_ctx, |ui| {
                // demo mixing of egui and Rustycan
                ui.label("Test");

                rustycan_ui! { ui
                    VertStack (
                        Label "Hello world" (before=1x after=1x) // center
                        HorizStack (
                            before_first=1x // right align
                            between=10

                            Button "Ok"
                            Button "Cancel"
                        )
                    )
                }
            });
        });

        // Draw things before egui

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}

// rustycan_ui! {
//     Grid(
//         cols={sizes=(50 2x 1x) between=40 before_first=10 after_last=20}
//         rows=1x
//         children_default_spacing=(1 1 1 2))
//     {
//         Button "Ok" (
//             style=cool_style
//             override_spacing=(10 10 10 10)
//             parent.rows=(1..2) parent.cols=1
//         ).ok
//         Button "Cancel" (
//             style=cool_style
//             override_spacing=(1 2 3 4)
//         ).cancel
//         Slider "Brush Size".brush_size (
//             range=(1..100)
//             value=50
//             override_spacing=(_,_,1x,1x)
//         ).my_slider
//     }
// }
