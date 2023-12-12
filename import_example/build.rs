use std::fmt::Write;

use wgsl_to_wgpu::{create_shader_module, WriteOptions, create_shader_module_with_imports};

fn main() {
    println!("cargo:rerun-if-changed=src/shader.wgsl");
    println!("cargo:rerun-if-changed=src/other.wgsl");
    let wgsl_source = std::fs::read_to_string("src/shader.wgsl").unwrap();
    let other_source = std::fs::read_to_string("src/other.wgsl").unwrap();
    let source = format!("{}\n{}", other_source, wgsl_source);

    // Generate the Rust bindings and write to a file.
    let mut text = String::new();
    writeln!(&mut text, "// File automatically generated by build.rs.").unwrap();
    writeln!(&mut text, "// Changes made to this file will not be saved.").unwrap();
    text += &create_shader_module(
        &other_source,
        "other.wgsl",
        WriteOptions {
            derive_bytemuck: true,
            ..Default::default()
        },
    )
    .unwrap();
    std::fs::write("src/other.rs", text.as_bytes()).unwrap();

    // Generate the Rust bindings and write to a file.
    let mut text = String::new();
    writeln!(&mut text, "// File automatically generated by build.rs.").unwrap();
    writeln!(&mut text, "// Changes made to this file will not be saved.").unwrap();
    text += &create_shader_module_with_imports(
        &source,
        Vec::from([
            ("super::other".to_string(), other_source)
        ]),
        WriteOptions {
            derive_bytemuck: true,
            ..Default::default()
        },
    )
    .unwrap();
    std::fs::write("src/shader.rs", text.as_bytes()).unwrap();
}
