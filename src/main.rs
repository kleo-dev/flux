use std::path::Path;
use flux_rt::run_py;

fn main() {
    run_py(Path::new("bro.py"));
}
