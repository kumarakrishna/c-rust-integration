// Declare that this is a Rust library.
#![crate_type = "cdylib"]

// Include any necessary external libraries or modules.
// use libc;

// Import necessary modules and functions.
// mod addlib;
mod utillib;

// Re-export the functions that your C code will call.
// pub use addlib::add;
pub use utillib::write_to_stderr;

pub use utillib::hexdump;
