# Dangling Pointer Bug in Rust

This repository demonstrates a common error in Rust involving dangling pointers when working with vectors and raw pointers.  The code creates a vector, gets a raw pointer to its data, and then reassigns the vector, creating a dangling pointer.  Accessing the memory via the dangling pointer results in undefined behavior.

The `bug.rs` file contains the buggy code, while `bugSolution.rs` demonstrates how to avoid this problem.

**To Reproduce:**

1. Clone this repository.
2. Navigate to the directory.
3. Compile and run the `bug.rs` file (it will likely crash or produce unpredictable results).
4. Compile and run the `bugSolution.rs` file to see the corrected version.

This example highlights the importance of careful memory management when working with raw pointers in Rust.  Always ensure that the data pointed to by a raw pointer remains valid for the pointer's lifetime.  Using safe Rust abstractions, such as references, is generally preferred unless absolutely necessary to work with raw pointers.