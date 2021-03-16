use crate::node::SlotValue;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AddNode;

crate::node! {
    AddNode => "Add Node"(ctx: &NodeCtx, a: Sound, b: Sound) -> [out: Sound] {
        out = SlotValue::Sound(a.unwrap_f64() + b.unwrap_f64());
    }

    fn ui(ui: &mut Ui) -> bool {
        false
    }
}
