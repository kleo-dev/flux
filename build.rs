fn main() {
    let lib_dir = std::process::Command::new("python3")
        .arg("-c")
        .arg("import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")
        .output()
        .expect("failed to run python3")
        .stdout;

    let lib_dir = String::from_utf8(lib_dir).unwrap();
    println!("cargo:rustc-link-search=native={}", lib_dir.trim());

    println!("cargo:rustc-link-lib=python3.13");
}
