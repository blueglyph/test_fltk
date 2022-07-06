use std::env;
use std::path::Path;

fn main() {
    println!("OUT_DIR = {}", env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=src/ui.fl");
    let g = fl2rust::Generator::default();
    let src_filename = Path::new("src").join("ui.fl");
    let dst_filename = Path::new("src").join("ui.rs");
    println!("generating {} -> {}", &src_filename.to_str().unwrap(), &dst_filename.to_str().unwrap());
    g.in_out_with_directives_preamble(&src_filename, &dst_filename).expect("Failed to generate rust from fl file!");
}
