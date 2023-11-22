# c-rust-integration

## Members

### Group No. 1
- Arnav Gupta (2021A7PS2092G)
- Ashman Mehra (2021A7PS2508G)
- Kumarakrishna Valeti (2021A7PS2617G)
- Joel Tony (2021A7PS2077G)

## Problem Statement

We aim to explore conditional memory safety guarantees and performance impacts of integrating Rust code into a C codebase. While Rust's memory safety guarantees are well-documented, it's often impractical to port entire codebases to Rust. Existing studies typically compare complete Rust-based codebases with complete C-based ones, which may not reflect real-world scenarios for legacy codebases. Documentation on Rust and C integration approaches is sparse, and roadblocks and solutions are not well-documented. Our goal is to study integration approaches that provide maximum memory safety guarantees and performance improvements.

## Software Architecture

We chose Microps as a case study—a GitHub project offering an educational implementation of a TCP/IP protocol stack. The repository contains various source code files, each handling specific aspects of network communication.

- `arp.c`: Address Resolution Protocol (ARP).
- `ether.c`: Ethernet protocol-related functions.
- `icmp.c`: Internet Control Message Protocol (ICMP) messages.
- `ip.c`: Internet Protocol (IP) code.
- `net.c`: Central component for integrating various protocol layers.
- `sock.c`: Socket operations for network communication from the application layer.
- `tcp.c`: Transmission Control Protocol (TCP) implementation.
- `udp.c`: User Datagram Protocol (UDP) handling.
- `utils.c`: Utility functions, including a queue data structure and file/data handling.

We ported `utils.c` to Rust with a focus on providing conditional memory safety guarantees by managing memory allocation and access in Rust.

## Repo Structure

- `/doc`: Milestone 1 ppt, project proposal, README, Errors_avoided.pdf.
- `/tests`: Test cases.
- `/code_orig`: utils_rust cargo project , rust_functions.h, rust_queue_check
- `/code-external`: Original Microps code.
- `/results`: Benchmarking results.

## Distribution of Tasks

- Porting of utils.c to Rust - Ashman Mehra, Kumarakrishna Valeti, Arnav Gupta
- Identification of errors avoided - Arnav Gupta
- Integration of utils_rust with microps_icmp - Kumarakrishna Valeti
- Integration of Queue with test C code - Ashman Mehra
- Benchmarking and testing - Joel Tony
- README - Kumarakrishna Valeti, Ashman Mehra, Joel Tony

## RUST and C Integration Pipeline
This section explains the complete end-to-end integration pipelin of rust into c modules using FFI safe practices. This section eplains the complete procedure using the integration example in [`integration/integration_examples`](https://github.com/kumarakrishna/c-rust-integration/tree/c6fd00c157ab592c559dd5928c5a0c4b86febbbd/code_orig/integration_examples)

We will use the following directory structure to integerate the c and rust code. The c code can be found in `ccode` folder while the rust code is stored in `rustcode`

    .
    ├── ccode
    │   ├── bin
    |   |   ├── myapp
    |   |   ├── error.txt
    |   |   ├── hexdump.txt
    │   ├── include
    |   |   ├── mylibrary.h
    │   └── build.sh
    ├── rustcode
    │   ├── src
    |   |   ├── utlilib
    |   |   |   ├── mod.rs
    |   ├── target
    |   |   |   ├── ....
    |   ├── lib.rs
    │   └── cargo.toml
    └── README.md

### 1. Translating the c code in rust
This part involves translating the c code in rust. This part is particularly challenging because very often the in-build c library functions are not available in rust and even if they are, the process to use them is very different. The translation process is hence not very staright-forward. One needs to divide the code into blocks, understand what each each piece of code takes in and gves as the output and then find the equivalent rust code for it. The in-built libraries also need to be implemented. Here is an example of a error handling function which can also be found in [`/rustcode/src/utillib/mod.rs`](https://github.com/kumarakrishna/c-rust-integration/blob/c6fd00c157ab592c559dd5928c5a0c4b86febbbd/code_orig/integration_examples/rustcode/src/utillib/mod.rs):

```rust
#[no_mangle]
pub extern "C" fn write_to_stderr(message: *const c_char) -> u32 {
    // Safety: Ensure that the message pointer is valid and points to a null-terminated C string.
    let message_cstr =  CStr::from_ptr(message);

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
```

Notice that there are some changes from the typical rust code we write. We have to use FFI or Foreign Function Interface mechanism. This means that the input to the functions are not the usual data types but rather pointers. We need to type-caste the pointers to rust data types. For example, in the code above we have used `c_char` pointer (for message) which is a raw OS pointer and the cast into a rust string using `CStr`. Additionally, we have to set function names to `#[no_mangle]` to ensure that the rust compiler does not change the function names during compilation. This way, our c code can directly use the names we have specified.

### 2. Exposing the functions as a shared library
After the code is translated into rust and thoroughly tested for equivalence, we export them as a shared library. In our running example, we have placed the rust code in [`utillib`](https://github.com/kumarakrishna/c-rust-integration/tree/c6fd00c157ab592c559dd5928c5a0c4b86febbbd/code_orig/integration_examples/rustcode/src/utillib). Now to expose the functions, we build a `lib.rs` file that declares the library to exported and the functions:

```rust
// Declare that this is a Rust library.
#![crate_type = "cdylib"]

// Include any necessary external libraries or modules.
use libc;

// Import necessary modules and functions.
mod utillib;

// Re-export the functions that your C code will call.
pub use utillib::write_to_stderr;
```

The `#![crate_type = "cdylib"]` is an attribute in Rust code that specifies the type of crate that is being built. In this case, it indicates that the Rust code is intended to be compiled into a dynamic library (shared library) using the Rust compiler (rustc). We also include the necessary `libc` functions used (if any). Then import the library and export the methods individually as shown above.

### 3. Interfacing the shared library in c header file(s)
Since the functions to be used in the c code are not avilable within the c file, we need header files to interface the rust and c code. This header can be found in [/ccode/include](https://github.com/kumarakrishna/c-rust-integration/tree/c6fd00c157ab592c559dd5928c5a0c4b86febbbd/code_orig/integration_examples/ccode/include) A typical header file should like like:

```c
#ifndef MYLIBRARY_H_
#define MYLIBRARY_H_

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

uint32_t write_to_stderr(const char* message);
#endif
```

Notice that the return types (as specified in rust code as well) are native in nature. For instance, here the return type is `uint32_t` from the `stdint.h` library instead of the standard `int` because `uint32_t` is FFI safe. 

### 4. Modifying the function signatures in c
This part inviolves scrapping through the c code and finding all the code blocks that use the translated code, changing the data types and modifying the function calls according the ones defined in Step 2. Here is an example which can also be found in [/ccode/src/main.c](https://github.com/kumarakrishna/c-rust-integration/blob/c6fd00c157ab592c559dd5928c5a0c4b86febbbd/code_orig/integration_examples/ccode/src/main.c):

```c
int main() {

    const char* message = "This is a message from C BUT written by rust.";
    int result = write_to_stderr(message);
}
```

Note that we have to convert the message into a `char*` pointer and the result is extracted in an int (u32int_t to int is handled by c compiler). This part is relatively easy as only data type matching and modification needs to be done.

### 5. Integrating shared library at compile time
For the final step, we have to create a `build` file. The command to integrate the dynamic shared rust libraru with c is:

```shell
gcc src/main.c -Iinclude/ -L../rustcode/target/release -lutillib -o bin/myapp
```

The build file can be found in [/ccode/build.sh](https://github.com/kumarakrishna/c-rust-integration/blob/c6fd00c157ab592c559dd5928c5a0c4b86febbbd/code_orig/integration_examples/ccode/build.sh) The `-L` flag is used to specify the location of the rustc compiled library and the `-l` specifies the library to be used in the location. We store the final integrated object file in the `bin` folder which can be run as `./app`

## Tests and Benchmarking

- [@Joel]: Mention any relevant information about tests and benchmarking here.


## Future Work

- [List any planned future work or improvements]
