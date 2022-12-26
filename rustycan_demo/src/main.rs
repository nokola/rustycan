use rustycan_macro::rustycan_ui;

fn main() {
    rustycan_ui! {
        Grid(
            cols={sizes=(50 2x 1x) between=40 before_first=10 after_last=20}
            rows=1x
            children_default_spacing=(1 1 1 2))
        {
            Button "Ok" (
                style=cool_style
                override_spacing=(10 10 10 10)
                parent.rows=(1..2) parent.cols=1
            ).ok
            Button "Cancel" (
                style=cool_style
                override_spacing=(1 2 3 4)
            ).cancel
            Slider "Brush Size".brush_size (
                range=(1..100)
                value=50
                override_spacing=(_,_,1x,1x)
            ).my_slider
        }
    }

    println!("Hello, world!");
}
