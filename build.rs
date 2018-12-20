// extern crate bindgen;

// use std::env;
// use std::path::PathBuf;

// fn main() {
//     let bindings = bindgen::Builder::default()
//         .header("napi/wrapper.h")
//         .whitelist_type("napi.*")
//         .whitelist_function("napi.*")
//         .rustified_enum(".*")
//         .clang_arg("-DNAPI_EXPERIMENTAL")
//         .generate()
//         .expect("Unable to generate bindings");

//     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//     bindings.write_to_file(out_path.join("bindings.rs"))
//             .expect("Couldn't write bindings!");
// }

fn main() {

}
