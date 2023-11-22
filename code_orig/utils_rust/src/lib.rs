use std::io::{self, Write};
use std::fs::File;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::collections::VecDeque;
use std::marker::PhantomData;

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

    0
}

const BIG_ENDIAN: i32 = 4321;
const LITTLE_ENDIAN: i32 = 1234;

static mut ENDIAN: i32 = 0;

fn byteorder() -> i32 {
    let x: u32 = 0x00000001;
    unsafe {
        if (*(x as *const u8)) == 1 {
            LITTLE_ENDIAN.try_into().unwrap()
        } else {
            BIG_ENDIAN.try_into().unwrap()
        }
    }
}

fn byteswap16(v: u16) -> u16 {
    ((v & 0x00ff) << 8) | ((v & 0xff00) >> 8)
}

fn byteswap32(v: u32) -> u32 {
    ((v & 0x000000ff) << 24) | ((v & 0x0000ff00) << 8) | ((v & 0x00ff0000) >> 8) | ((v & 0xff000000) >> 24)
}

#[no_mangle]
pub extern "C" fn ntoh16(n: u16) -> u16 {
    unsafe {
        if ENDIAN == 0 {
            ENDIAN = byteorder();
        }
        if ENDIAN == LITTLE_ENDIAN {
            byteswap16(n)
        } else {
            n
        }
    }
}

#[no_mangle]
pub extern "C" fn ntoh32(n: u32) -> u32 {
    unsafe {
        if ENDIAN == 0 {
            ENDIAN = byteorder();
        }
        if ENDIAN == LITTLE_ENDIAN {
            byteswap32(n)
        } else {
            n
        }
    }
}

#[no_mangle]
pub extern "C" fn cksum16(addr: *const u16, count: usize, init: u32) -> u16 {
    let mut sum = init;
    let mut addr = addr;

    unsafe {
        let mut remaining = count;

        while remaining > 1 {
            sum = sum.wrapping_add(*addr as u32);
            addr = addr.offset(1);
            remaining -= 2;
        }

        if remaining == 1 {
            sum = sum.wrapping_add(*(addr as *const u8) as u32);
        }

        while sum >> 16 != 0 {
            sum = (sum & 0xffff) + (sum >> 16);
        }

        !sum as u16
    }
}

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


#[allow(unused_attributes)]
#[allow(dead_code)]
#[no_mangle]
struct QueueEntry<T> {
    next: Option<Box<QueueEntry<T>>>,
    data: T,
    _marker: PhantomData<T>,
}

#[allow(unused_attributes)]
#[allow(dead_code)]
struct Queue<T> {
    queue: VecDeque<QueueEntry<T>>,
    _marker: PhantomData<T>,
}
 
#[allow(dead_code)]
impl<T> QueueEntry<T> {
    fn new(data: T) -> Self {
        QueueEntry {
            next: None,
            data,
            _marker: PhantomData,
        }
    }
}

#[allow(dead_code)]
impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            queue: VecDeque::new(),
            _marker: PhantomData,
        }
    }
 
    fn push(&mut self, data: QueueEntry<T>) {
        self.queue.push_back(data);
    }
 
    fn pop(&mut self) -> Option<QueueEntry<T>> {
        self.queue.pop_front()
    }
 
    fn peek(&self) -> Option<&T> {
        self.queue.front().map(|entry| &entry.data)
    }
 
    fn foreach<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut T),
    {
        for entry in &mut self.queue {
            func(&mut entry.data);
        }
    }
}