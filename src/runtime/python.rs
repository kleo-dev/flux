use std::fs;

use crate::FluxModule;
use cpython::{GILGuard, ObjectProtocol, PyObject, PyTuple, Python, PythonObject, ToPyObject};

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

pub struct PythonModule<'a> {
    path: String,
    python: Python<'a>,
}

impl FluxModule for PythonModule<'_> {
    fn init(&self) {
        let code = fs::read_to_string(&self.path).unwrap();
        self.python.run(&code, None, None).unwrap();
    }

    fn call(&self, func: &str, args: Vec<&dyn crate::Serialize>) -> crate::FluxValue {
        let module = self.python.import("__main__").unwrap();
        let func = module.get(self.python, func).unwrap();

        let py_args: Vec<PyObject> = args
            .iter()
            .map(|a| match a.serialize() {
                crate::FluxValue::Int(i) => i.to_py_object(self.python).into_object(),
                crate::FluxValue::Float(f) => f.to_py_object(self.python).into_object(),
                crate::FluxValue::Str(s) => s.to_py_object(self.python).into_object(),
                crate::FluxValue::Null => self.python.None(),
            })
            .collect();

        let args_tuple = PyTuple::new(self.python, &py_args);

        let result: PyObject = func.call(self.python, args_tuple, None).unwrap();

        if let Ok(i) = result.extract::<i64>(self.python) {
            crate::FluxValue::Int(i)
        } else if let Ok(f) = result.extract::<f64>(self.python) {
            crate::FluxValue::Float(f)
        } else if let Ok(s) = result.extract::<String>(self.python) {
            crate::FluxValue::Str(s)
        } else {
            crate::FluxValue::Null
        }
    }

    fn end(&self) {}
}
