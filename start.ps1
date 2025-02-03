# Set the PATH environment variable
$dllPath = "C:/s2o/s2o_net_lib/src/windivert"  # Replace this with the actual path to your DLL directory
$env:PATH += ";$dllPath"

# Run your Rust program
cargo run  # Replace with the actual name of your compiled Rust executable