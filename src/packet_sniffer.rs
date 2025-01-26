use std::ffi::CString;
use std::sync::{Arc, Mutex};
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

type InitSniffer = unsafe extern "C" fn() -> i32;
type RunSniffer = unsafe extern "C" fn(callback: extern "C" fn(data: *const u8, length: i32));
type CapturePacketData = unsafe extern "C" fn() -> i32;
type GetCapturedPacketCount = unsafe extern "C" fn() -> i32;
type GetCapturedPacket = unsafe extern "C" fn(index: i32) -> *const u8;

static CAPTURED_PACKETS: Lazy<Arc<Mutex<Vec<Vec<u8>>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

extern "C" fn packet_callback(data: *const u8, length: i32) {
    println!("Packet captured: length = {}", length);
    if length > 0 {
        let packet = unsafe { std::slice::from_raw_parts(data, length as usize) };
        let mut packets = CAPTURED_PACKETS.lock().expect("Failed to lock captured packets");
        packets.push(packet.to_vec());
    }
}

static LIB: Lazy<Result<Arc<Library>, PacketSnifferError>> = Lazy::new(|| {
    unsafe { 
        Library::new("C:/S2O/s2o_net_lib/src/s2o_dll/packet_sniffer.dll")
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

static INIT_SNIFFER: Lazy<Result<InitSniffer, PacketSnifferError>> = Lazy::new(|| get_function!(b"init_sniffer\0", InitSniffer));
static RUN_SNIFFER: Lazy<Result<RunSniffer, PacketSnifferError>> = Lazy::new(|| get_function!(b"run_sniffer\0", RunSniffer));
static CAPTURE_PACKET_DATA: Lazy<Result<CapturePacketData, PacketSnifferError>> = Lazy::new(|| get_function!(b"capture_packet_data\0", CapturePacketData));
static GET_CAPTURED_PACKET_COUNT: Lazy<Result<GetCapturedPacketCount, PacketSnifferError>> = Lazy::new(|| get_function!(b"get_captured_packet_count\0", GetCapturedPacketCount));
static GET_CAPTURED_PACKET: Lazy<Result<GetCapturedPacket, PacketSnifferError>> = Lazy::new(|| get_function!(b"get_captured_packet\0", GetCapturedPacket));

pub fn start_packet_sniffer(_log_buffers: &LogBuffers) -> Result<(), PacketSnifferError> {
    let init_sniffer = INIT_SNIFFER.as_ref().map_err(|e| e.clone())?;
    let result = unsafe { init_sniffer() };
    if result != 0 {
        return Err(PacketSnifferError::FunctionLoadError(libloading::Error::DlOpen {
            desc: CString::new("Failed to start packet sniffer.").unwrap().as_c_str().into()
        }));
    }

    let run_sniffer = RUN_SNIFFER.as_ref().map_err(|e| e.clone())?;
    unsafe { run_sniffer(packet_callback) };
    Ok(())
}

pub fn stop_packet_sniffer(_log_buffers: &LogBuffers) -> Result<(), PacketSnifferError> {
    // Implement stop logic if needed
    Ok(())
}

pub fn capture_packet_data(_log_buffers: &LogBuffers) -> Result<(), PacketSnifferError> {
    let capture_data = CAPTURE_PACKET_DATA.as_ref().map_err(|e| e.clone())?;
    unsafe { capture_data() };
    Ok(())
}

pub fn get_captured_packet_count(_log_buffers: &LogBuffers) -> Result<i32, PacketSnifferError> {
    let get_count = GET_CAPTURED_PACKET_COUNT.as_ref().map_err(|e| e.clone())?;
    let count = unsafe { get_count() };
    Ok(count)
}

pub fn get_captured_packet(index: i32, _log_buffers: &LogBuffers) -> Result<Option<Vec<u8>>, PacketSnifferError> {
    let get_packet = GET_CAPTURED_PACKET.as_ref().map_err(|e| e.clone())?;
    let packet_ptr = unsafe { get_packet(index) };
    if packet_ptr.is_null() {
        Ok(None)
    } else {
        // Assuming a fixed maximum length for the packets
        let packet_length = 65536; // Adjust this if necessary
        let packet = unsafe { Vec::from_raw_parts(packet_ptr as *mut u8, packet_length, packet_length) };
        Ok(Some(packet))
    }
}

pub fn human_readable_packet_data(packet_data: &[u8]) -> String {
    let hex_content = packet_data.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    let ascii_content = packet_data.iter()
        .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
        .collect::<String>();

    format!("Hex: {}\nASCII: {}", hex_content, ascii_content)
}
