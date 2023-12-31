# Possible errors when using malloc - dealloc

## Memory Leaks

A memory leak occurs when you allocate memory in a program and then never deallocate it. This can happen for a number of reasons, such as forgetting to call the `dealloc()` function or losing a reference to the memory block. Memory leaks can cause your program to slow down or even crash, as they can lead to the depletion of available memory.
Rust’s ownership system and garbage collector take care of memory management automatically. The ownership model ensures that when the scope ends, the object is automatically destroyed, and its memory is reclaimed. The garbage collector automatically identifies data structures like Vec and String and deallocates memory that is no longer referenced.

## Double Freeing and Accessing Freed Memory

Double freeing occurs when you try to deallocate the same memory block twice using the `dealloc()` function, corrupting the memory heap and leading to unpredictable behavior, including crashes, memory corruption, and security vulnerabilities. Accessing freed memory occurs when we try to access memory locations or data structures that have been freed up, leading to segmentation faults and other unpredictable crashes.
Rust's ownership system and borrowing rules prevent double freeing by ensuring that there is always a single owner of each piece of data. When an object is created, it is owned by the scope in which it is created. When the scope ends, the object is automatically destroyed, and its memory is reclaimed. This means that it is impossible to deallocate an object twice, as it will have already been destroyed by the time you try to deallocate it again. Rust also transfers ownership of objects to prevent double freeing using the `dealloc()` function.

## Using a Memory Block After It's Been Reallocated

This error occurs when you try to access a memory block that has moved to a different memory address/location. This can happen if you store a pointer to the original memory block and then reallocate the block to a new location. The old pointer will still point to the original location of the memory block, which is now no longer valid. This can lead to data corruption, crashes, and other unpredictable behavior.
Rust prevents this error by ensuring that pointers are always valid when used. The Rust compiler enforces lifetime rules, preventing the use of pointers after their corresponding objects have been destroyed. When a memory block is reallocated, the Rust compiler automatically updates any pointers that point to the original memory block to point to the new location of the memory block. This ensures that pointers are always valid and that you can never use a pointer to access a memory block that has been reallocated.

# Possible errors when using a queue data structure

## Overflow

Overflow occurs when you try to enqueue an element into a full queue. A queue is a data structure that follows a First-In-First-Out (FIFO) order, meaning that elements are removed in the same order they were inserted. When the queue is full, there is no more space to store new elements. Attempting to enqueue an element into a full queue will result in an overflow error.
In C, an overflow error can manifest as a segmentation fault or a program crash. This is because the `enqueue()` function will try to access memory that is beyond the bounds of the queue's allocated space. Segmentation faults are a type of error that occurs when a program attempts to access memory that it is not permitted to access.

## Underflow

Underflow occurs when you try to dequeue an element from an empty queue. Dequeuing an element from an empty queue means that there are no more elements to remove. Attempting to dequeue an element from an empty queue will result in an underflow error.
In C, an underflow error can manifest as a segmentation fault or a program crash. This is because the `dequeue()` function will try to access memory that is beyond the bounds of the queue's allocated space. Segmentation faults are a type of error that occurs when a program attempts to access memory that it is not permitted to access.

## Segmentation Fault

A segmentation fault can occur when you try to access memory that is not allocated to the queue. This can happen for several reasons, such as accessing an array index out of bounds or accessing memory that has been freed. Segmentation faults can also occur when you try to modify a string literal or attempt to read or write to memory beyond the bounds of an array.

## How Rust Solves These Errors

Rust employs several mechanisms to prevent overflow, underflow, and segmentation fault errors when using the queue data structure:

- **Ownership system:** Rust's ownership system ensures that there is always a single owner of each piece of data. When an object is created, it is owned by the scope in which it is created. When the scope ends, the object is automatically destroyed, and its memory is reclaimed. This prevents dangling pointers and memory leaks, which can lead to segmentation faults.
- **Borrowing rules:** Rust's borrowing rules ensure that references to objects are always valid. When you borrow an object, you are essentially given temporary access to it. The borrowed reference must be valid for the entire duration of its use, preventing dangling pointers and segmentation faults.
- **Lifetime checks:** Rust's compiler enforces lifetime rules, which specify the validity period of references. This prevents the use of references after their corresponding objects have been destroyed, eliminating the possibility of accessing invalid memory and segmentation faults.
- **Bounds checking:** Rust automatically performs bounds checking on arrays, preventing index out of bounds errors and segmentation faults.
- **Safe data types:** Rust provides safe data types, such as `Vec` and `String`, which manage memory allocation and deallocation automatically. This eliminates the need for manual memory management and reduces the risk of memory-related errors.

# Handling Errors Explicitly Using `Ok()` and `Err()` to Prevent Unwanted Compile-Time Errors

## Explicit Error Handling with Result Type

The `Result` type in Rust is a powerful mechanism for representing either a successful outcome or an error. It encapsulates either an `Ok` value, indicating a successful result, or an `Err` value, indicating an error. This explicit representation of errors encourages developers to handle potential failures proactively.

## Preventing Unwanted Compile-Time Errors

Rust's type system enforces error handling by requiring developers to explicitly handle both successful and error cases when working with the `Result` type. This prevents unwanted compile-time errors that could arise from ignoring potential failures.

