use std::io::{self, Write};
use std::os::unix::io::{FromRawFd, AsRawFd};
use std::fs::File;
use libc::FILE;
use libc::fileno;

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

