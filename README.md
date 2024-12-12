# Data Race in Rust
This repository demonstrates a simple example of a data race in Rust.  Data races occur when multiple threads or parts of a program access and modify the same memory location concurrently without proper synchronization. This can lead to unpredictable and inconsistent behavior.

The `bug.rs` file contains the buggy code. The `bugSolution.rs` file demonstrates how to fix the data race using appropriate synchronization mechanisms (in this case, a mutex).

## How to run
1. Clone the repository: `git clone <repo_url>`
2. Navigate to the repository: `cd <repo_name>`
3. Compile and run the buggy code: `rustc bug.rs && ./bug`
4. Compile and run the solution code: `rustc bugSolution.rs && ./bugSolution`

Observe the different outputs of the buggy and corrected code.