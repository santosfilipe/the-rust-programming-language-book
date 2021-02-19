# Error Handling

- Rust groups errors into two major categories: recoverable and unrecoverable errors:
  - For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation.
  
  - Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

- Rust doesn’t have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error.

## Unrecoverable Errors with panic!

- When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

- The call to `panic!` causes the error message contained in the last two lines. The first line shows our panic message and the place in our source code where the panic occurred

## Using a panic! Backtrace

- To protect your program from buffer overread vulnerabilities, if you try to read an element at an index that doesn’t exist, Rust will stop execution and refuse to continue. 

`thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5 note: run with "RUST_BACKTRACE=1" environment variable to display a backtrace`

- This error points at a file we didn’t write, `libcore/slice/mod.rs`. That’s the implementation of `slice` in the Rust source code. The code that gets run when we use `[]` on our vector `v` is in `libcore/slice/mod.rs`, and that is where the `panic!` is actually happening.

- The next note line tells us that we can set the `RUST_BACKTRACE` environment variable to get a backtrace of exactly what happened to cause the error. A backtrace is a list of all the functions that have been called to get to this point:

`$ RUST_BACKTRACE=1 cargo run`

## Recoverable Errors with Result

- The `Result` enum is defined as having two variants, `Ok` and `Err`, as follows:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

- Here we tell Rust that when the result is `Ok`, return the inner file value out of the `Ok` variant. The other arm of the `match` handles the case where we get an `Err` value from `File::open`. In this example, we’ve chosen to call the `panic!` macro. If there’s no file named `hello.txt` in our current directory and we run this code, we’ll see the output from the panic! macro.

## Matching on Different Errors

