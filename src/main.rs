use flux_rt::{runtime::python::PythonModule, prelude::*};

fn main() {
    let bro = PythonModule("bro.py".to_string());
    bro.init();

    bro.call("hello", vec![]);
}
