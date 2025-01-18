extern crate libc;
use libc::c_char;
use std::ffi::CString;

#[link(name = "packet_sniffer", kind = "dylib")]
extern "C" {
    fn start_sniffer(ip_address: *const c_char);
}

pub fn start_packet_sniffer(ip: &str) {
    let c_ip = CString::new(ip).unwrap();
    unsafe {
        start_sniffer(c_ip.as_ptr());
    }
}
