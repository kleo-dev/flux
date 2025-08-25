use std::collections::{BTreeMap, HashMap};

use crate::{Deserialize, FluxValue, Serialize};

impl Serialize for i64 {
    fn serialize(&self) -> FluxValue {
        FluxValue::Int(*self)
    }
}

impl Serialize for f64 {
    fn serialize(&self) -> FluxValue {
        FluxValue::Float(self.to_bits())
    }
}

impl Deserialize for f64 {
    fn deserialize(v: FluxValue) -> Self {
        match v {
            FluxValue::Float(bits) => f64::from_bits(bits),
            v => panic!("Expected Int, instead got {v:?}"),
        }
    }
}

impl Serialize for &str {
    fn serialize(&self) -> FluxValue {
        FluxValue::Str(self.to_string())
    }
}

impl Serialize for String {
    fn serialize(&self) -> FluxValue {
        FluxValue::Str(self.clone())
    }
}

impl Deserialize for i64 {
    fn deserialize(v: FluxValue) -> Self {
        match v {
            FluxValue::Int(i) => i,
            v => panic!("Expected Int, instead got {v:?}"),
        }
    }
}

impl<K: Serialize, V: Serialize> Serialize for HashMap<K, V> {
    fn serialize(&self) -> FluxValue {
        FluxValue::Map(
            self.iter()
                .map(|(k, v)| (k.serialize(), v.serialize()))
                .collect::<BTreeMap<FluxValue, FluxValue>>(),
        )
    }
}

impl<K: Deserialize + Ord, V: Deserialize> Deserialize for BTreeMap<K, V> {
    fn deserialize(v: FluxValue) -> Self {
        match v {
            FluxValue::Map(m) => m
                .iter()
                .map(|(k, v)| (K::deserialize(k.clone()), V::deserialize(v.clone())))
                .collect::<BTreeMap<K, V>>(),
            v => panic!("Expected Int, instead got {v:?}"),
        }
    }
}
