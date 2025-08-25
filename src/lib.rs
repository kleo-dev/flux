use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FluxValue {
    Int(i64),
    Float(u64),
    Str(String),
    Null,
    Array(Vec<FluxValue>),
    Map(BTreeMap<FluxValue, FluxValue>),
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

pub mod runtime;
pub mod values;

pub mod prelude {
    pub use crate::{Deserialize, FluxModule, Serialize};
}

impl FluxValue {
    pub fn extract<T: Deserialize>(self) -> T {
        T::deserialize(self)
    }
}
