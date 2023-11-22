use std::io::{self, Write};
use std::fs::File;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::collections::VecDeque;
use core::marker::PhantomData;
use libc::FILE;
use libc::fileno;
use std::os::fd::FromRawFd;

#[no_mangle]
pub extern "C" fn hexdump_rust(fp: *mut FILE, data: *const u8, len: usize) -> io::Result<()> {
    let mut offset = 0;
    
    // Cast the raw pointers to slices
    let data_slice = unsafe { std::slice::from_raw_parts(data, len) };

    // Convert the FILE pointer to a mutable Write reference
    let mut file = unsafe { File::from_raw_fd(fileno(fp)) };

    write!(&mut file, "+------+-------------------------------------------------+------------------+\n").unwrap();

    while offset < len {
        write!(&mut file, "| {:04x} | ", offset).unwrap();

        for index in 0..16 {
            if offset + index < len {
                write!(&mut file, "{:02x} ", data_slice[offset + index]).unwrap();
            } else {
                write!(&mut file, "   ").unwrap();
            }
        }

        write!(&mut file, "| ").unwrap();

        for index in 0..16 {
            if offset + index < len {
                if data_slice[offset + index].is_ascii() && data_slice[offset + index].is_ascii_graphic() {
                    write!(&mut file, "{}", data_slice[offset + index] as char).unwrap();
                } else {
                    write!(&mut file, ".").unwrap();
                }
            } else {
                write!(&mut file, " ").unwrap();
            }
        }

        write!(&mut file, " |\n").unwrap();

        offset += 16;
    }

    write!(&mut file, "+------+-------------------------------------------------+------------------+\n").unwrap();

    Ok(())
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
pub extern "C" fn ntoh16_rust(n: u16) -> u16 {
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
pub extern "C" fn ntoh32_rust(n: u32) -> u32 {
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
pub extern "C" fn cksum16_rust(addr: *const u16, count: usize, init: u32) -> u16 {
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
