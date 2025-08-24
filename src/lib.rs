use std::{ffi::CString, fs, path::Path};

#[link(name = "python3.13")]
extern "C" {
    fn Py_Initialize();
    fn PyRun_SimpleString(command: *const i8) -> i32;
    fn Py_Finalize();
}

pub fn run_py(path: &Path) {
    let code = fs::read_to_string(path).expect("Failed to read python script");

    unsafe {
        Py_Initialize();

        let code = CString::new(code.as_str()).unwrap();
        PyRun_SimpleString(code.as_ptr());

        Py_Finalize();
    }
}