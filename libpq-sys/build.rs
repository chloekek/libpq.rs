use std::{env::var_os, path::PathBuf, process::Command};

fn main()
{
    let out_dir = PathBuf::from(var_os("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=wrapper.h");
    let status =
        Command::new("bindgen")
        .arg("--rustified-enum").arg(".*")
        .arg("--output").arg(out_dir.join("bindings.rs"))
        .arg("wrapper.h")
        .status().unwrap();
    assert!(status.success());
}
