use std::io::{self, Write};
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::fs::File;

#[no_mangle]
pub extern "C" fn write_to_stderr(message: *const c_char) -> u32 {
    // Safety: Ensure that the message pointer is valid and points to a null-terminated C string.
    let message_cstr = unsafe { CStr::from_ptr(message) };

    if let Ok(message_str) = message_cstr.to_str() {
        let stderr = io::stderr();
        let mut handle = stderr.lock();

        if let Err(e) = writeln!(&mut handle, "{}", message_str) {
            eprintln!("Error writing to stderr: {}", e);
            return 1; // Return a non-zero value to indicate an error.
        }
        return 0; // Return 0 to indicate success.
    } else {
        eprintln!("Error converting message to a valid string.");
        return 1; // Return a non-zero value to indicate an error.
    }
}

#[no_mangle]
pub extern "C" fn hexdump(fp: *mut c_void, data: *const u8, data_len: usize) -> i32 {
    // Check for null pointers, if necessary
    if fp.is_null() || data.is_null() {
        return libc::EINVAL;
    }

    // Cast the fp back to the correct type
    let writer_ptr: *mut Box<dyn Write> = fp as *mut Box<dyn Write>;

    // Create a safe reference to the Write trait object from the raw pointer.
    let _writer: &mut dyn Write = unsafe {
        &mut *writer_ptr
    };

    let mut offset = 0;
    let mut _index: u32;

    let mut file = File::create("./bin/output.txt").unwrap();
    // let text_to_write = "Hello, this is some text written to a file.";

    if let Err(_) = file.write_all("+------+-------------------------------------------------+------------------+\n".as_bytes()) {
        return libc::EFAULT; // Return an error code
    }

    while offset < data_len {
        
        let mut formatted_output = format!("| {:04x} | ", offset);
        if let Err(_) = file.write_all(formatted_output.as_bytes()) {
            return libc::EFAULT;
        }

        for index in 0..16 {
            if offset + index < data_len {
                formatted_output = format!("{:02x} ", unsafe { *data.add((offset + index) as usize) });
                if let Err(_) = file.write_all(formatted_output.as_bytes()) {
                    return libc::EFAULT;
                }
            } else if let Err(_) = file.write_all("   ".as_bytes()) {
                return libc::EFAULT;
            }
        }

        if let Err(_) = file.write_all("| ".as_bytes()) {
            return libc::EFAULT;
        }

        for index in 0..16 {
            if offset + index < data_len {
                if let Some(byte) = unsafe { data.add((offset + index) as usize).as_ref() } {
                    if byte.is_ascii() && byte.is_ascii_graphic() {
                        formatted_output = format!("{} ", *byte as char);
                        if let Err(_) = file.write_all(formatted_output.as_bytes()) {
                            return libc::EFAULT;
                        }
                    } else if let Err(_) = file.write_all(".".as_bytes()) {
                        return libc::EFAULT;
                    }
                }
            } else if let Err(_) = file.write_all(" ".as_bytes()) {
                return libc::EFAULT;
            }
        }

        if let Err(_) = file.write_all(" |\n".as_bytes()) {
            return libc::EFAULT;
        }

        offset += 16;
    }

    if let Err(_) = file.write_all("+------+-------------------------------------------------+------------------+\n".as_bytes()) {
        return libc::EFAULT;
    }

    return 0;
    
}

