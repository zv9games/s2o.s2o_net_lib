fn main() {
    // Specify the absolute directory containing the packet_sniffer.lib file
    println!("cargo:rustc-link-search=native=C:/S2O/s2o_net_lib/s2o_dll");
    // Link to the packet_sniffer library
    println!("cargo:rustc-link-lib=dylib=packet_sniffer");
}
