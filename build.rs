extern crate bindgen;
extern crate gcc;

use std::env;
use std::path::PathBuf;


fn main() {
	gcc::compile_library("libyoga.a", &["src/c/YGEnums.c", "src/c/YGNodeList.c", "src/c/Yoga.c"]);

	let bindings = bindgen::Builder::default()
		.no_unstable_rust()
		.hide_type("max_align_t") // This fails `cargo test` so disable for now
		.header("src/c/wrapper.h")
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

	bindings.write_to_file(out_path.join("bindings.rs"))
		.expect("Unable to write bindings!");
}