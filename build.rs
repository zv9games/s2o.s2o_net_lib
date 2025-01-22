use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use CARGO_MANIFEST_DIR to get the project root
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let dll_dir = PathBuf::from(manifest_dir).join("src/s2o_dll");

    // Add the DLL and LIB directory to the PATH and LIB environment variables
    let old_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{};{}", dll_dir.display(), old_path);
    env::set_var("PATH", &new_path);

    let old_lib = env::var("LIB").unwrap_or_default();
    let new_lib = format!("{};{}", dll_dir.display(), old_lib);
    env::set_var("LIB", &new_lib);

    // Print information for debugging
    println!("cargo:rerun-if-changed=src/s2o_dll");
    println!("cargo:warning=Setting PATH to: {}", new_path);
    println!("cargo:warning=Setting LIB to: {}", new_lib);

    // Verify and print DLL and LIB paths
    let dll_path = dll_dir.join("packet_sniffer.dll");
    let lib_path = dll_dir.join("packet_sniffer.lib");

    if dll_path.exists() {
        println!("cargo:warning=Found packet_sniffer.dll in {:?}", dll_dir);
    } else {
        println!("cargo:warning=packet_sniffer.dll not found in {:?}", dll_dir);
    }

    if lib_path.exists() {
        println!("cargo:warning=Found packet_sniffer.lib in {:?}", dll_dir);
        // Link the packet_sniffer.lib explicitly
        println!("cargo:rustc-link-lib=static=packet_sniffer");
        println!("cargo:rustc-link-search=native={}", dll_dir.display());
    } else {
        println!("cargo:warning=packet_sniffer.lib not found in {:?}", dll_dir);
    }

    Ok(())
}