use std::fs;

use crate::{FluxModule, FluxValue};
use cpython::{
    GILGuard, ObjectProtocol, PyDict, PyList, PyObject, PyTuple, Python, PythonObject, ToPyObject,
};

pub fn import<'a>(path: &str, gil: &'a GILGuard) -> PythonModule<'a> {
    let m = PythonModule {
        path: path.to_string(),
        python: gil.python(),
    };
    m.init();
    m
}

pub fn get_gil() -> GILGuard {
    Python::acquire_gil()
}

pub fn flux_to_object(py: Python<'_>, flux: &FluxValue) -> PyObject {
    match flux {
        crate::FluxValue::Null => py.None(),
        crate::FluxValue::Int(i) => i.to_py_object(py).into_object(),
        crate::FluxValue::Float(f) => f.to_py_object(py).into_object(),
        crate::FluxValue::Str(s) => s.to_py_object(py).into_object(),
        crate::FluxValue::Array(a) => PyList::new(
            py,
            &a.iter()
                .map(|f| flux_to_object(py, f))
                .collect::<Vec<PyObject>>(),
        )
        .into_object(),
        crate::FluxValue::Map(a) => {
            let map = PyDict::new(py);
            for (k, v) in a {
                map.set_item(py, flux_to_object(py, k), flux_to_object(py, v))
                    .unwrap();
            }
            map.into_object()
        }
    }
}

pub struct PythonModule<'a> {
    path: String,
    python: Python<'a>,
}

impl FluxModule for PythonModule<'_> {
    fn init(&self) {
        let code = fs::read_to_string(&self.path).unwrap();
        self.python.run(&code, None, None).unwrap();
    }

    fn call(&self, func: &str, args: Vec<&dyn crate::ToFlux>) -> crate::FluxValue {
        let module = self.python.import("__main__").unwrap();
        let func = module.get(self.python, func).unwrap();

        let py_args: Vec<PyObject> = args
            .iter()
            .map(|a| flux_to_object(self.python, &a.to_flux()))
            .collect();

        let args_tuple = PyTuple::new(self.python, &py_args);

        let result: PyObject = func.call(self.python, args_tuple, None).unwrap();

        to_flux_value(self.python, &result)
    }

    fn end(&self) {}
}

fn to_flux_value(py: Python, result: &PyObject) -> crate::FluxValue {
    if let Ok(i) = result.extract::<i64>(py) {
        crate::FluxValue::Int(i)
    } else if let Ok(f) = result.extract::<f64>(py) {
        crate::FluxValue::Float(f.to_bits())
    } else if let Ok(s) = result.extract::<String>(py) {
        crate::FluxValue::Str(s)
    } else if let Ok(list) = result.cast_as::<PyList>(py) {
        let mut items = Vec::new();
        for item in list.iter(py) {
            items.push(to_flux_value(py, &item));
        }
        crate::FluxValue::Array(items)
    } else if let Ok(dict) = result.cast_as::<PyDict>(py) {
        let mut map = std::collections::BTreeMap::new(); // or HashMap if you don’t need ordering
        for (k, v) in dict.items(py) {
            let key = to_flux_value(py, &k);
            let val = to_flux_value(py, &v);
            map.insert(key, val);
        }
        crate::FluxValue::Map(map)
    } else {
        crate::FluxValue::Null
    }
}
