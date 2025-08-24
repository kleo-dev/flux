use std::{ffi::{CString, c_char}, fs};

    use crate::{FluxModule, FluxValue, Serialize};

    #[link(name = "python3.13")]
    extern "C" {
        fn Py_Initialize();
        fn Py_Finalize();

        fn PyRun_SimpleString(command: *const i8) -> i32;

        fn PyImport_AddModule(name: *const c_char) -> *mut PyObject;
        fn PyModule_GetDict(module: *mut PyObject) -> *mut PyObject;
        fn PyDict_GetItemString(dict: *mut PyObject, key: *const c_char) -> *mut PyObject;

        fn PyObject_CallObject(callable: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
        fn Py_BuildValue(format: *const c_char, ...) -> *mut PyObject;

        fn PyLong_AsLong(obj: *mut PyObject) -> i64;
    }

    #[repr(C)]
    pub struct PyObject;

    pub struct PythonModule(pub String);

    impl FluxModule for PythonModule {
        fn init(&self) {
            let code = fs::read_to_string(&self.0).expect("Failed to read python script");
            let code = CString::new(code.as_str()).unwrap();

            unsafe {
                Py_Initialize();
                PyRun_SimpleString(code.as_ptr());
            }
        }

        fn call(&self, func: &str, args: Vec<&dyn Serialize>) -> FluxValue {
            use std::ffi::CString;

            unsafe {
                // 1. Get __main__ module dict
                let main_str = CString::new("__main__").unwrap();
                let module = PyImport_AddModule(main_str.as_ptr());
                let dict = PyModule_GetDict(module);

                // 2. Get function by name
                let func_name = CString::new(func).unwrap();
                let func_obj = PyDict_GetItemString(dict, func_name.as_ptr());

                if func_obj.is_null() {
                    panic!("Python function '{}' not found", func);
                }

                let mut fmt = String::from("(");
                let values: Vec<FluxValue> = args.iter().map(|a| a.serialize()).collect();

                for v in &values {
                    match v {
                        FluxValue::Int(_) => fmt.push('l'),   // long
                        FluxValue::Float(_) => fmt.push('d'), // double
                        FluxValue::Str(_) => fmt.push('s'),   // string
                        FluxValue::Null => fmt.push('O'),  // PyObject*
                    }
                }
                fmt.push(')');

                let fmt_c = CString::new(fmt).unwrap();

                // Call Py_BuildValue with the right arguments
                let args_obj: *mut PyObject = match values.as_slice() {
                    [FluxValue::Int(a), FluxValue::Int(b)] => {
                        Py_BuildValue(fmt_c.as_ptr(), *a, *b)
                    }
                    [FluxValue::Float(a), FluxValue::Float(b)] => {
                        Py_BuildValue(fmt_c.as_ptr(), *a, *b)
                    }
                    [FluxValue::Str(a), FluxValue::Str(b)] => {
                        let a_c = CString::new(a.as_str()).unwrap();
                        let b_c = CString::new(b.as_str()).unwrap();
                        Py_BuildValue(fmt_c.as_ptr(), a_c.as_ptr(), b_c.as_ptr())
                    }
                    [FluxValue::Int(a)] => {
                        Py_BuildValue(fmt_c.as_ptr(), *a)
                    }
                    [FluxValue::Float(a)] => {
                        Py_BuildValue(fmt_c.as_ptr(), *a)
                    }
                    [FluxValue::Str(a)] => {
                        let a_c = CString::new(a.as_str()).unwrap();
                        Py_BuildValue(fmt_c.as_ptr(), a_c.as_ptr())
                    }
                    _ => std::ptr::null_mut(),
                };

                let result_obj = PyObject_CallObject(func_obj, args_obj);

                if result_obj.is_null() {
                    FluxValue::Null
                } else {
                    let value = PyLong_AsLong(result_obj);
                    FluxValue::Int(value)
                }
            }
        }


        fn end(&self) {
            unsafe {
                Py_Finalize();
            }
        }
    }