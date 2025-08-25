use flux_rt::{
    runtime::python::{self},
    FluxModule,
};

fn main() {
    let gil = python::get_gil();
    let m = python::import("bro.py", &gil);

    println!("{:?}", m.call("hello", vec![&"leo"]).extract::<String>());

    println!("{:?}", m.call("add", vec![&10, &20]).extract::<i32>());
}
