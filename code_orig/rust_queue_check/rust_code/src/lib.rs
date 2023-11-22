// Declare that this is a Rust library.
#![crate_type = "cdylib"]

// Include any necessary external libraries or modules.
// use libc;

// Import necessary modules and functions.
// mod addlib;
mod utillib;

// Re-export the functions that your C code will call.
// pub use addlib::add;

pub use utillib::queue_entry;
pub use utillib::queue_head;
pub use utillib::queue_init;
pub use utillib::create_queue_head;
pub use utillib::queue_push;
pub use utillib::queue_pop;
pub use utillib::queue_peek;