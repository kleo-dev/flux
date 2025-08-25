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

pub trait ToFlux {
    fn to_flux(&self) -> FluxValue;
}

pub trait FluxModule {
    fn init(&self);
    fn end(&self);
    fn call(&self, func: &str, args: Vec<&dyn ToFlux>) -> FluxValue;
}

pub mod runtime;
pub mod values;

pub mod prelude {
    pub use crate::{FluxModule, ToFlux};
}

impl FluxValue {
    pub fn extract<T: From<FluxValue>>(self) -> T {
        T::from(self)
    }
}
