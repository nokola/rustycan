pub trait Adapter {
    fn button(&self) {}
    fn label(&self, _text: &str) {}
    fn layout<SomeType>(&self) {}
}

pub struct NullUi;
impl Adapter for NullUi {}
