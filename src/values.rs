use crate::{Deserialize, FluxValue, Serialize};

impl Serialize for i64 {
    fn serialize(&self) -> FluxValue {
        FluxValue::Int(*self)
    }
}
impl Serialize for f64 {
    fn serialize(&self) -> FluxValue {
        FluxValue::Float(*self)
    }
}
impl Serialize for &str {
    fn serialize(&self) -> FluxValue {
        FluxValue::Str(self.to_string())
    }
}
impl Deserialize for i64 {
    fn deserialize(v: FluxValue) -> Self {
        match v {
            FluxValue::Int(i) => i,
            _ => panic!("Expected Int"),
        }
    }
}
