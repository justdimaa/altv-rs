extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::{Read, Write};

const CAPI_DIR: &'static str = "thirdparty/altv-capi-server";

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!(
        "cargo:rustc-link-search=native={}/{}/lib",
        project_dir, CAPI_DIR
    );
    println!(r"cargo:rustc-link-lib=dylib=altv-capi-server");

    let bindings = bindgen::Builder::default()
        .header(format!("{}/include/altv-capi-predefines.h", CAPI_DIR))
        .header(format!("{}/include/server/altv-capi.h", CAPI_DIR))
        .header(format!("{}/include/altv-capi-extra.h", CAPI_DIR))
        .clang_arg("-DALT_SERVER_API")
        .clang_arg("-std=c++17")
        .clang_arg("-xc++")
        .rustified_enum("*")
        .generate()
        .expect("Unable to generate server bindings");

    bindings
        .write_to_file(project_dir.clone() + "/src/natives.rs")
        .expect("Couldn't write server bindings!");

    let mut file = File::open(project_dir.clone() + "/src/natives.rs").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    drop(file);

    let data = data.replace(" nullptr_t", " std_nullptr_t");
    let mut file = File::create(project_dir.clone() + "/src/natives.rs").unwrap();
    file.write(data.as_bytes()).unwrap();
}
