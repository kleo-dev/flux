#[derive(Debug)]
pub enum FluxValue {
    Int(i64),
    Float(f64),
    Str(String),
    Null,
    // later: List(Vec<FluxValue>), Dict(HashMap<String, FluxValue>)
}

pub trait Serialize {
    fn serialize(&self) -> FluxValue;
}

pub trait Deserialize {
    fn deserialize(v: FluxValue) -> Self;
}

pub trait FluxModule {
    fn init(&self);
    fn end(&self);
    fn call(&self, func: &str, args: Vec<&dyn Serialize>) -> FluxValue;
}

pub mod values;
pub mod runtime;

pub mod prelude {
    pub use crate::{FluxModule, Serialize, Deserialize};
}
