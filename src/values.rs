use std::collections::{BTreeMap, HashMap};

use crate::{FluxValue, ToFlux};

impl ToFlux for i64 {
    fn to_flux(&self) -> FluxValue {
        FluxValue::Int(*self)
    }
}

impl ToFlux for f64 {
    fn to_flux(&self) -> FluxValue {
        FluxValue::Float(self.to_bits())
    }
}

impl From<FluxValue> for f64 {
    fn from(value: FluxValue) -> Self {
        match value {
            FluxValue::Float(bits) => f64::from_bits(bits),
            v => panic!("Expected Int, instead got {v:?}"),
        }
    }
}

impl ToFlux for &str {
    fn to_flux(&self) -> FluxValue {
        FluxValue::Str(self.to_string())
    }
}

impl ToFlux for String {
    fn to_flux(&self) -> FluxValue {
        FluxValue::Str(self.clone())
    }
}

impl From<FluxValue> for i64 {
    fn from(value: FluxValue) -> Self {
        match value {
            FluxValue::Int(i) => i,
            v => panic!("Expected Int, instead got {v:?}"),
        }
    }
}

impl<K: ToFlux, V: ToFlux> ToFlux for HashMap<K, V> {
    fn to_flux(&self) -> FluxValue {
        FluxValue::Map(
            self.iter()
                .map(|(k, v)| (k.to_flux(), v.to_flux()))
                .collect::<BTreeMap<FluxValue, FluxValue>>(),
        )
    }
}

impl<K: From<FluxValue> + Ord, V: From<FluxValue>> From<FluxValue> for BTreeMap<K, V> {
    fn from(value: FluxValue) -> Self {
        match value {
            FluxValue::Map(m) => m
                .iter()
                .map(|(k, v)| (K::from(k.clone()), V::from(v.clone())))
                .collect::<BTreeMap<K, V>>(),
            v => panic!("Expected Int, instead got {v:?}"),
        }
    }
}
