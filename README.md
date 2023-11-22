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

### Repo Structure

- `/code-external`: Original Microps code
- `/code_orig`: utils_rust, rust_functions.h, rust_queue_check, integration_examples
- `/doc`: Milestone 1 ppt, project proposal, README, Errors_avoided.pdf.
- `/tests`: Test cases.
- `/results`: Benchmarking results
- `/microps_integration`: microps_integrated, utils_rust

### Distribution of Tasks

- Porting of utils.c to Rust - Ashman Mehra, Kumarakrishna Valeti, Arnav Gupta
- Identification of errors avoided - Arnav Gupta
- Integration of utils_rust with microps_icmp - Kumarakrishna Valeti
- Integration of Queue with test C code - Ashman Mehra
- Benchmarking and testing - Joel Tony
- README - Kumarakrishna Valeti, Ashman Mehra, Arnav Gupta

### utils_rust
This cargo project contains the ported utils.c code. To compile and generate the dynamic shared library run 'cargo build --release' in root of utils_rust. The share library should be generated as libutillib.dylib **on mac** (.so on linux) under target/release.
/src/utillib/mod.rs contains the ported utils.c code
/cargo.toml specifies the dependencies and instruction to compile it as a dynamic library

### rust_queue_check
[@Ashman]

### microps_integration
The dependency of icmp.c on utils.c has been replaced with utils_rust. The rust code was compiled into a dynamic shared library as explain in utils_rust. The file rust_functions.h contains extern definitions of rust functions, which are imported in icmp.c. 'rust_functions.c' is included in icmp.c and the makefile was edited to link the rust library. Makefile was updated to link the rust library during compilation of icmp.c.

**shared library in linux has a .so extension and .dylib extension on mac. For running on linux/mac check the extension sign in Makefile for Rust_library.**

### integration_examples
Contains an example integration of Rust into C code. Explained in detail below.

### Errors_avoided.pdf
Contains errors avoided by porting to Rust. Listed in the document Errors_avoided.pdf under docs.

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


## POPL Aspects
Here are some Principles of Programming language-specific aspects present in the code, along with their corresponding line numbers:

1. Ownership and Borrow Checker:
  Rust's ownership system ensures memory safety by tracking resource ownership and preventing data races through strict borrowing rules.
   - Line 19: `let mut file = unsafe { File::from_raw_fd(fileno(fp)) };`
   - Line 16: `let data_slice = unsafe { std::slice::from_raw_parts(data, len) };`
   - Line 21: `write!(&mut file, "+------+-------------------------------------------------+------------------+\n").unwrap();`
   - Line 24: `write!(&mut file, "| {:04x} | ", offset).unwrap();`

2. Type Safety:
   Rust's strong static typing helps catch type-related errors at compile-time, reducing runtime errors and enhancing code reliability.   
   - Line 19: `let mut file: File = unsafe { File::from_raw_fd(fileno(fp)) };`
   - Line 16: `let data_slice: &[u8] = unsafe { std::slice::from_raw_parts(data, len) };`
   - Lines 39: Usage of `data_slice[offset + index] as char`.

3. Ownership Transfer (Move Semantics):
   Rust's move semantics prevent accidental data aliasing and enable efficient memory management by transferring ownership instead of copying data.
   - Line 19: `let mut file = unsafe { File::from_raw_fd(fileno(fp)) };`
   - Line 113: `let mut addr = addr;`

4. Static Mutability:
   Rust's static mut allows controlled mutable access to global state, maintaining safety by ensuring exclusive access through unsafe blocks.
   - Line 61: `static mut ENDIAN: i32 = 0;`

5. Enums and Pattern Matching:
    Rust's enums and pattern matching facilitate expressive and exhaustive handling of different states, reducing the likelihood of handling unexpected cases.
   - Line 26, 36: `for index in 0..16 { ... }` - Iterating with a pattern matching loop.

6. Lifetimes:
    Lifetimes in Rust enable precise control over references, preventing dangling references and ensuring that borrowed data remains valid throughout its usage   
   - Line 184 - 186: `fn foreach<F>(&mut self, mut func: F) where F: FnMut(&mut T),`

7. Struct Generics and PhantomData:
   Rust's use of generics in structs allows for flexible and reusable data structures, and PhantomData helps express type variance without affecting runtime behavior.
   - Lines 139-143: `struct QueueEntry<T> { ... }`
   - Lines 146-150: `struct Queue<T> { ... }`
   - Line 154-160: `fn new(data: T) -> Self { ... }`
   - Line 172: `fn push(&mut self, data: QueueEntry<T>) { ... }`
   - Line 180: `fn peek(&self) -> Option<&T> { ... }`

8. Error Handling:
   Rust's Result and Option types, along with the try_into method, promote explicit and structured error handling, enhancing code clarity and reliability.
   - Line 67: `LITTLE_ENDIAN.try_into().unwrap()`
   - Line 69: `BIG_ENDIAN.try_into().unwrap()`

9. Unwrapping Results and Option:
    The use of unwrap in Rust signifies explicit handling of success and failure cases, encouraging developers to consider and handle potential errors, contributing to safer code.
   - Line 21: `write!(&mut file, "+------+-------------------------------------------------+------------------+\n").unwrap();`
   - Line 67: `LITTLE_ENDIAN.try_into().unwrap()`
   - Line 69: `BIG_ENDIAN.try_into().unwrap()`
   - Line 181: `self.queue.front().map(|entry| &entry.data)`

These aspects highlight some of the unique features and characteristics of Rust compared to C, allowing Rust to be a safer language.

## Issues Faced

### 1. Conversion to different appropriate data types
Data types in Rust are more expressive and safe as compared to C data types, allowing the user to have greater control over the boundaries of what each variable, function or traits can hold or access. Rust has a much stronger type system, with more data types available than those in C code.This means the selection of the data type is a critical factor to keep in mind while porting code from C to Rust. 
 
Selecting improper data types initially led to a domino effect of errors across multiple file levels where each function calls another one and uses its result according to its type signature. Correcting these was possible after a more in depth study about rust, its data types, what it signifies and where its use cases primarily are 
 
Line number 12 in microps_integration/utils_rust/src/utillib/mod.rs: using usize as a data type for the return type io::Result<usize> gives a compile time error.

### 2. Manipulating raw pointers, converting them to appropriate pointer types in Rust to utilize the Rust paradigm of ownership and borrowchecker to the fullest
Data is passed from Rust to C using raw pointers which then need to be converted to data types in the Rust codebase to be used as variables. Rust provides multiple types of pointers, few common ones being
- Box<T>
- Rc<T>
- Arc<T>
- Cell<T>
- RefCell<T>
 
Each being suitable for a certain kind of operation. For example, Rc<T> is primarily used when we want shared access to variables where as Cell<T> is primarily used to provide interior mutability access. Each has a different use case and careful thought has to go into using the appropriate pointer type otherwise the code can get extremely clumsly and proceeding further becomes a mammoth challenge.

### 3. Finding appropriate crates to replicate the C libraries
Libraries in C are a colection of pre-compiled code that provide functions and data types to be used by the programmer, they can be static or dynamic depending on when they are linked and loaded into the memory. Crates in rust are also pre-code compiled code which can be used by the programmer, they can be 'binary' which can be executed directly or 'library' crates which are used as dependencies as other crates
 
Finding the appropriate crates for our project to replicate the functionalities of the libraries used in the original Microps codebase was a time consuming process where we had to try out multiple crates from https://crates.io/ website before arriving on a crate which we deemed suitable
 
Line number 1-8 of microps_integration/utils_rust/src/utillib/mod.rs : appropriate crates found to replicate the functionality of libraries

### 4. Integrating codebases using the makefile
Integrating the newly written Rust code with the existing C code requires multiple steps. 
1. Compiling the rust code to create using "cargo build" to create a binary
2. Locating where the binary is stored
3. Adding the binary to the correct C files using the "#include<>" statement
4. Modifying the makefile to incorporate the rust code file
5. Compiling the C code which now contains the Rust FFI
 
Major issues in locating and including the build file into C code. The build file needs changes after the "cargo build" step in order to fully merge with the C. identification and solving of this issue was a major issue when integrating codebases
 
Writing the correct makefile, integrating C and Rust code at correct locations with all needed dependencies was an issue that the team faced as well. 

## Other possible intergation issues

### 1. Uninitialized variables
C let's the programmer compile code with uninitialized variables. It also lets uninitialized variables be passed as arguments to function calls, which can lead to null pointers or segmentation fault if the programmer is not careful
 
Rust does not allow the programmer to pass uninitialized variables as function calls, in Rust code as well over the foreign function interface, making the ported code safer with fewer chances of bugs and/or errors
 
### 2. Segmentation fault
In C it is possible to access out of bounds indexes when using FFIs. RUST has compile time checks to prevent this from happening.  
 
When we declare an array of size n in C and try to access the n+1th index in RUST code which ideally should give a compile time error. But there is no compile time check and RUST accesses the memory location which results in runtime errors.
When we declare an array of size n in RUST and try to access the n+1th index in C code which ideally should give a compile time error. But there is no compile time check and C accesses the memory location which results in runtime errors

### 3. Overflow error
In C integer overflow errors commonly result in runtime errors. RUST has compile time checks in place to prevent this, making the code safer and error-free while improving runtime performance
 
We initialized an integer as INT_MAX in C before passing it to RUST and adding one to it and printed the values in RUST code. This leads to a compile time error, due to RUST safety principles.
 
We initialized an integer as INT_MAX in RUST before passing it to C and added one to it and printed the values in C code. This results in overflow.

### 4. Scope and lifetime of data structures passed between boundaries using an FFI
Porting C code to Rust project needs multiple modifications to lifetimes of variable, function signatures, etc to fully utilise the Rust model. Lifetimes are assigned after careful checks, making sure that variables and functions outside the scope are not able to access these data structures. 
 
## Tests and Benchmarking

### Perf
"perf" in Linux is a powerful performance analysis tool that helps benchmark and optimize system and application performance. It provides insights into CPU usage, memory operations, and other critical metrics, enabling developers to identify bottlenecks and improve overall system efficiency. With features like profiling and tracing, "perf" is instrumental in fine-tuning software for optimal execution.
@Joel

- [@Joel]:Pics
  
### Massif
"Massif" is another tool in the Valgrind suite, specifically designed for heap profiling in C and C++ programs. It helps benchmark and optimize memory usage by providing a detailed analysis of heap allocations and deallocations. Massif aids developers in identifying memory-related bottlenecks, allowing for targeted optimizations to improve overall program efficiency.

- [@Joel]:Pics

## Future Work

- Port more files to Rust and becnhmark changes with incremental increase in percentage of Rust in codebase.
- Provide a general framework to identify functions and files to port to Rust.
- Provide various approaches to integration of Rust into a C/C++ codebase.
- Provide conditional memory safety guarantees for Rust integrations and provide documentation on the same.
- Explore and document integration challeneges and ABI compatibility issues. Provide solutions for the same.
- Conduct a similar study with different languages, and provide a general framework to identify the suitablility of languages for a given task.
