use std::env;
use std::path::PathBuf;

fn main() {
	let manifest_dir: String = std::env::var("CARGO_MANIFEST_DIR").unwrap();

	println!{"cargo:rustc-link-search=native={}/src/C_SDK", manifest_dir}; 	// sdk dir
	println!{"cargo:rustc-link-lib=spcm"};			// .a file

	let target: String = std::env::var("TARGET").unwrap();

	let bindings: bindgen::Bindings = bindgen::Builder::default()
		.default_macro_constant_type(bindgen::MacroTypeVariation::Signed)	// use i32 default instead of u32
		.header("wrapper.h")
		.clang_arg(format!("--target={}", target))
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.generate()
		.expect("Failed to generate bindings.");

	let out_path: PathBuf = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Failed to write bindings to file.");
}