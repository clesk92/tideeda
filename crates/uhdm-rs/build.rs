fn main() {
    println!("cargo:rerun-if-changed=schema/uhdm.capnp");
    
    // Attempt to compile the schema.
    // In a real scenario, we would use the full uhdm.capnp from the official repo.
    // For now, we will use a placeholder if the file exists, or warn if it doesn't.
    if std::path::Path::new("schema/uhdm.capnp").exists() {
        capnpc::CompilerCommand::new()
            .file("schema/uhdm.capnp")
            .run()
            .expect("schema compiler command failed");
    } else {
        println!("cargo:warning=schema/uhdm.capnp not found. Please download it from the UHDM repository.");
    }
}
