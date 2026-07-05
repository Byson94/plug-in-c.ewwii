fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    match cbindgen::generate(crate_dir) {
        Ok(bindings) => {
            bindings.write_to_file("ewwii.h");
        }
        Err(e) => {
            println!("cargo:warning=cbindgen failed: {:?}", e);
        }
    }
}
