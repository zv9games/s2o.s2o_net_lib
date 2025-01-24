use std::sync::{Arc};
use std::ffi::CString;
use libloading::{Library, Symbol};
use once_cell::sync::Lazy;
use crate::logging::LogBuffers;

#[derive(Debug)]
pub enum PacketSnifferError {
    LibraryLoadError(libloading::Error),
    FunctionLoadError(libloading::Error),
    CStringConversionError(std::ffi::NulError),
}

impl Clone for PacketSnifferError {
    fn clone(&self) -> Self {
        match self {
            PacketSnifferError::LibraryLoadError(e) => PacketSnifferError::LibraryLoadError(libloading::Error::DlOpen {
                desc: CString::new(e.to_string()).unwrap().as_c_str().into()
            }),
            PacketSnifferError::FunctionLoadError(e) => PacketSnifferError::FunctionLoadError(libloading::Error::DlOpen {
                desc: CString::new(e.to_string()).unwrap().as_c_str().into()
            }),
            PacketSnifferError::CStringConversionError(e) => PacketSnifferError::CStringConversionError(e.clone()),
        }
    }
}

impl From<libloading::Error> for PacketSnifferError {
    fn from(err: libloading::Error) -> Self {
        PacketSnifferError::LibraryLoadError(err)
    }
}

impl From<std::ffi::NulError> for PacketSnifferError {
    fn from(err: std::ffi::NulError) -> Self {
        PacketSnifferError::CStringConversionError(err)
    }
}

impl std::fmt::Display for PacketSnifferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketSnifferError::LibraryLoadError(e) => write!(f, "Library load error: {}", e),
            PacketSnifferError::FunctionLoadError(e) => write!(f, "Function load error: {}", e),
            PacketSnifferError::CStringConversionError(e) => write!(f, "CString conversion error: {}", e),
        }
    }
}

impl std::error::Error for PacketSnifferError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PacketSnifferError::LibraryLoadError(e) => Some(e),
            PacketSnifferError::FunctionLoadError(e) => Some(e),
            PacketSnifferError::CStringConversionError(e) => Some(e),
        }
    }
}

impl From<&PacketSnifferError> for PacketSnifferError {
    fn from(e: &PacketSnifferError) -> Self {
        match e {
            PacketSnifferError::LibraryLoadError(e) => PacketSnifferError::LibraryLoadError(libloading::Error::DlOpen {
                desc: CString::new(e.to_string()).unwrap().as_c_str().into()
            }),
            PacketSnifferError::FunctionLoadError(e) => PacketSnifferError::FunctionLoadError(libloading::Error::DlOpen {
                desc: CString::new(e.to_string()).unwrap().as_c_str().into()
            }),
            PacketSnifferError::CStringConversionError(e) => PacketSnifferError::CStringConversionError(e.clone()),
        }
    }
}

type StartSniffer = unsafe extern "C" fn() -> i32;
type StopSniffer = unsafe extern "C" fn();
type GetPacket = unsafe extern "C" fn(i32) -> *const u8;
type GetPacketCount = unsafe extern "C" fn() -> i32;

static LIB: Lazy<Result<Arc<Library>, PacketSnifferError>> = Lazy::new(|| {
    unsafe { 
        Library::new("packet_sniffer.dll")
            .map(|lib| Arc::new(lib))
            .map_err(|e| PacketSnifferError::LibraryLoadError(e))
    }
});

macro_rules! get_function {
    ($name:expr, $type:ty) => {
        {
            let lib = LIB.as_ref().map_err(|e| e.clone())?;
            unsafe {
                let symbol: Symbol<$type> = lib.get($name).map_err(|e| PacketSnifferError::FunctionLoadError(e))?;
                Ok(*symbol)
            }
        }
    };
}

static START_SNIFFER: Lazy<Result<StartSniffer, PacketSnifferError>> = Lazy::new(|| get_function!(b"start_sniffer", StartSniffer));
static STOP_SNIFFER: Lazy<Result<StopSniffer, PacketSnifferError>> = Lazy::new(|| get_function!(b"stop_sniffer", StopSniffer));
static GET_PACKET: Lazy<Result<GetPacket, PacketSnifferError>> = Lazy::new(|| get_function!(b"get_packet", GetPacket));
static GET_PACKET_COUNT: Lazy<Result<GetPacketCount, PacketSnifferError>> = Lazy::new(|| get_function!(b"get_packet_count", GetPacketCount));

pub fn start_packet_sniffer(_log_buffers: &LogBuffers) -> Result<(), PacketSnifferError> {
    let start_sniffer = *START_SNIFFER.as_ref()?;
    let result = unsafe { start_sniffer() };
    if result != 0 {
        return Err(PacketSnifferError::FunctionLoadError(libloading::Error::DlOpen {
            desc: CString::new("Failed to start packet sniffer.").unwrap().as_c_str().into()
        }));
    }
    Ok(())
}

pub fn stop_packet_sniffer(_log_buffers: &LogBuffers) -> Result<(), PacketSnifferError> {
    let stop_sniffer = *STOP_SNIFFER.as_ref()?;
    unsafe { stop_sniffer() };
    Ok(())
}

pub fn get_captured_packet(index: i32, _log_buffers: &LogBuffers) -> Result<Option<Vec<u8>>, PacketSnifferError> {
    let get_packet = *GET_PACKET.as_ref()?;
    let packet_ptr = unsafe { get_packet(index) };
    if packet_ptr.is_null() {
        Ok(None)
    } else {
        let packet_length = unsafe {
            let mut len = 0;
            while *packet_ptr.offset(len) != 0 { 
                len += 1; 
                if len > 65535 { // Prevent infinite loop
                    return Err(PacketSnifferError::FunctionLoadError(libloading::Error::DlOpen {
                        desc: CString::new("Packet data is not null-terminated or too long.").unwrap().as_c_str().into()
                    }));
                }
            }
            len
        };
        let packet = unsafe { Vec::from_raw_parts(packet_ptr as *mut u8, packet_length as usize, packet_length as usize) };
        Ok(Some(packet))
    }
}

pub fn get_captured_packet_count(_log_buffers: &LogBuffers) -> Result<i32, PacketSnifferError> {
    let get_packet_count = *GET_PACKET_COUNT.as_ref()?;
    let count = unsafe { get_packet_count() };
    Ok(count)
}

pub fn capture_packet_data(_log_buffers: &LogBuffers) -> Result<(), PacketSnifferError> {
    // Implement packet capture logic here
    Ok(())
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
    
    if packet_data.len() >= 14 { // Assuming at least Ethernet header
        let ether_type = u16::from_be_bytes([packet_data[12], packet_data[13]]);
        match ether_type {
            0x0800 => result.push_str("\nIPv4 Packet\n"),
            0x86DD => result.push_str("\nIPv6 Packet\n"),
            _ => result.push_str(&format!("\nUnknown or unsupported protocol (EtherType: 0x{:04X})\n", ether_type)),
        }
    } else {
        result.push_str("\nPacket too short to determine protocol\n");
    }
    
    result
}