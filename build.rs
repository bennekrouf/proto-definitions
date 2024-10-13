use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Get the OUT_DIR environment variable at build time
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("OUT_DIR: {:?}", out_dir);

    // Define the proto files to compile
    let proto_files = [
        "minioc.proto",
        "codeg.proto",
        "matching.proto",
        "messenger.proto",
        "parquetb.proto",
        "query.proto",
        "tenantm.proto",
        "transformer.proto",
    ];

    // Define the include paths where the proto files are located
    let proto_include_paths = ["./proto"];

    // Create the descriptor set path for dynamic discovery
    let descriptor_path = out_dir.join("descriptor_set.bin"); // More generic name
    println!("Descriptor path: {:?}", descriptor_path);

    // Write the descriptor path to a Rust file that will be included later
    let descriptor_file_content = format!(
        "pub const DESCRIPTOR_PATH: &str = \"{}\";",
        descriptor_path.display()
    );

    // Generate `out_dir.rs` file in OUT_DIR
    let out_dir_rs_path = out_dir.join("out_dir.rs");
    fs::write(out_dir_rs_path, descriptor_file_content).expect("Failed to write out_dir.rs");

    // Configure and compile all the proto files
    tonic_build::configure()
        .file_descriptor_set_path(descriptor_path) // This will generate descriptor set for all proto files
        .compile_protos(
            &proto_files
                .iter()
                .map(|f| format!("./proto/{}", f))
                .collect::<Vec<String>>(),
            &proto_include_paths,
        )
        .unwrap_or_else(|e| panic!("Failed to compile proto files: {}", e));

    println!("Protobuf files compiled successfully!");
}
