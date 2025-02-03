use std::env;
use std::path::PathBuf;

fn main() {
    let _out_dir = env::var("OUT_DIR").unwrap(); // Prefixed with an underscore to indicate it's intentionally unused

    // Add src/windivert to the library search path
    println!("cargo:rustc-link-search=native=src/windivert");
    println!("cargo:rustc-link-lib=dylib=WinDivert");

    // Set the PATH environment variable for the build process
    let mut path = env::var("PATH").unwrap();
    let windivert_path = PathBuf::from("src/windivert").canonicalize().unwrap();
    path.push_str(&format!(";{}", windivert_path.display()));
    env::set_var("PATH", path);
}
