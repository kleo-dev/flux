use flux_rt::{runtime::python::PythonModule, prelude::*};

fn main() {
    let bro = PythonModule("bro.py".to_string());
    bro.init();

    println!("{:?}", bro.call("hello", vec![]));
}
