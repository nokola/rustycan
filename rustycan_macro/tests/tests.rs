use rustycan_core::adapter::{Adapter, NullUi};
use rustycan_macro::rustycan_ui;

#[test]
fn compile_empty() {
    rustycan_ui! {}
}

#[test]
fn compile_basic() {
    let ui = NullUi;
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
}
