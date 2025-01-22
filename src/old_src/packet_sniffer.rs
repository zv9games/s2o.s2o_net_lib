use std::ffi::CString;
use libloading::Library;
use once_cell::sync::Lazy;
use std::sync::Arc;

#[derive(Debug)]
pub enum PacketSnifferError {
    LibraryLoadError(Arc<libloading::Error>),
    FunctionLoadError(Arc<libloading::Error>),
    CStringConversionError(std::ffi::NulError),
}

// Implementing `From` trait for conversions to `PacketSnifferError`
impl From<libloading::Error> for PacketSnifferError {
    fn from(err: libloading::Error) -> Self {
        PacketSnifferError::LibraryLoadError(Arc::new(err))
    }
}

impl From<std::ffi::NulError> for PacketSnifferError {
    fn from(err: std::ffi::NulError) -> Self {
        PacketSnifferError::CStringConversionError(err)
    }
}

// Implementing `From` for references to `PacketSnifferError`
impl From<&PacketSnifferError> for PacketSnifferError {
    fn from(err: &PacketSnifferError) -> Self {
        match err {
            PacketSnifferError::LibraryLoadError(e) => PacketSnifferError::LibraryLoadError(Arc::clone(e)),
            PacketSnifferError::FunctionLoadError(e) => PacketSnifferError::FunctionLoadError(Arc::clone(e)),
            PacketSnifferError::CStringConversionError(e) => PacketSnifferError::CStringConversionError(e.clone()),
        }
    }
}

// Implementing `Clone` for `PacketSnifferError`
impl Clone for PacketSnifferError {
    fn clone(&self) -> Self {
        match self {
            PacketSnifferError::LibraryLoadError(e) => PacketSnifferError::LibraryLoadError(Arc::clone(e)),
            PacketSnifferError::FunctionLoadError(e) => PacketSnifferError::FunctionLoadError(Arc::clone(e)),
            PacketSnifferError::CStringConversionError(e) => PacketSnifferError::CStringConversionError(e.clone()),
        }
    }
}

// FFI function pointers
type StartSniffer = unsafe extern "C" fn() -> i32; // Updated to match C++ signature
type CapturePacket = unsafe extern "C" fn() -> i32;
type StopSniffer = unsafe extern "C" fn();
type GetPacket = unsafe extern "C" fn(i32) -> *const i8;
type GetPacketCount = unsafe extern "C" fn() -> i32;

// Load the DLL and obtain function pointers
static LIB: Lazy<Result<Library, PacketSnifferError>> = Lazy::new(|| {
    unsafe { 
        Library::new("src/s2o_dll/packet_sniffer.dll")
            .map_err(|e| PacketSnifferError::LibraryLoadError(Arc::new(e)))
    }
});

macro_rules! get_function {
    ($name:expr, $type:ty) => {
        {
            let lib = LIB.as_ref().map_err(|e| e.clone())?;
            unsafe {
                lib.get::<$type>($name)
                    .map(|symbol| *symbol)
                    .map_err(|e| PacketSnifferError::FunctionLoadError(Arc::new(e)))
            }
        }
    };
}

static START_SNIFFER: Lazy<Result<StartSniffer, PacketSnifferError>> = Lazy::new(|| get_function!(b"start_sniffer", StartSniffer));
static CAPTURE_PACKET: Lazy<Result<CapturePacket, PacketSnifferError>> = Lazy::new(|| get_function!(b"capture_packet", CapturePacket));
static STOP_SNIFFER: Lazy<Result<StopSniffer, PacketSnifferError>> = Lazy::new(|| get_function!(b"stop_sniffer", StopSniffer));
static GET_PACKET: Lazy<Result<GetPacket, PacketSnifferError>> = Lazy::new(|| get_function!(b"get_packet", GetPacket));
static GET_PACKET_COUNT: Lazy<Result<GetPacketCount, PacketSnifferError>> = Lazy::new(|| get_function!(b"get_packet_count", GetPacketCount));

// Start the packet sniffer
pub fn start_packet_sniffer() -> Result<i32, PacketSnifferError> { // Updated to match C++ signature
    let start_sniffer = START_SNIFFER.as_ref()?;
    Ok(unsafe { start_sniffer() })
}

// Capture packet data
pub fn capture_packet_data() -> Result<(), PacketSnifferError> {
    let capture_packet = CAPTURE_PACKET.as_ref()?;
    let result = unsafe { capture_packet() };
    if result != 0 {
        let error_message = format!("Failed to capture packet. Return code: {}", result);
        eprintln!("{}", error_message);
        Err(PacketSnifferError::FunctionLoadError(Arc::new(libloading::Error::DlOpen { 
            desc: CString::new(error_message).unwrap().as_c_str().into() 
        })))
    } else {
        Ok(())
    }
}

// Stop the packet sniffer
pub fn stop_packet_sniffer() -> Result<(), PacketSnifferError> {
    let stop_sniffer = STOP_SNIFFER.as_ref()?;
    unsafe { stop_sniffer() };
    Ok(())
}

// Get a captured packet by index (returning bytes instead of string for raw data)
pub fn get_captured_packet(index: i32) -> Result<Option<Vec<u8>>, PacketSnifferError> {
    let get_packet = GET_PACKET.as_ref()?;
    let packet_ptr = unsafe { get_packet(index) };
    if packet_ptr.is_null() {
        Ok(None)
    } else {
        let packet = unsafe { std::slice::from_raw_parts(packet_ptr as *const u8, usize::MAX) };
        let end = packet.iter().position(|&b| b == 0).unwrap_or(packet.len());
        Ok(Some(packet[..end].to_vec()))
    }
}

// Get the count of captured packets
pub fn get_captured_packet_count() -> Result<i32, PacketSnifferError> {
    let get_packet_count = GET_PACKET_COUNT.as_ref()?;
    Ok(unsafe { get_packet_count() })
}

// Helper function to convert raw packet data to a more readable format
pub fn format_packet_data(packet_data: &[u8]) -> String {
    let hex_content = packet_data.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    let ascii_content = packet_data.iter()
        .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
        .collect::<String>();

    format!("Hex: {}\nASCII: {}", hex_content, ascii_content)
}

// Function to convert packet to human-readable format, including packet details
pub fn human_readable_packet_data(packet_data: &[u8]) -> String {
    let mut result = String::new();
    result.push_str(&format!("Packet Length: {} bytes\n", packet_data.len()));
    result.push_str(&format_packet_data(packet_data));
    
    // Here you could add more detailed parsing for different protocols if needed
    // For example, for Ethernet and IP headers:
    if packet_data.len() >= 14 { // Assuming at least Ethernet header
        let ether_type = u16::from_be_bytes([packet_data[12], packet_data[13]]);
        match ether_type {
            0x0800 => { // IPv4
                if packet_data.len() >= 34 { // Assuming at least IPv4 header after Ethernet
                    result.push_str("\nIPv4 Packet:\n");
                    // Add more detailed IPv4 parsing here
                }
            },
            0x86DD => { // IPv6
                if packet_data.len() >= 54 { // Assuming at least IPv6 header after Ethernet
                    result.push_str("\nIPv6 Packet:\n");
                    // Add more detailed IPv6 parsing here
                }
            },
            _ => result.push_str("\nUnknown or unsupported protocol\n"),
        }
    }
    
    result
}

// Example usage for converting to human-readable:
/*
match get_captured_packet(index) {
    Ok(Some(packet)) => {
        println!("Human Readable Packet Data:\n{}", human_readable_packet_data(&packet));
    },
    Ok(None) => println!("No packet at index {}", index),
    Err(e) => eprintln!("Error fetching packet: {:?}", e),
}
*/