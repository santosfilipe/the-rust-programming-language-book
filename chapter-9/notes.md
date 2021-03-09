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

- It's quite important to define different actions in a `match` expression depending on the exact error returned. 

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

- The type of the value that `File::open` returns inside the `Err` variant is `io::Error`, which is a struct provided by the standard library. This struct has a method kind that we can call to get an `io::ErrorKind` value. The enum `io::ErrorKind` is provided by the standard library and has variants representing the different kinds of errors that might result from an `io` operation.

## Shortcuts for Panic on Error: unwrap and expect

- Using `match` works well, but it can be a bit verbose. The `Result<T, E>` type has many helper methods defined on it to do various tasks. One of those methods, called `unwrap`. If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us. 

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

- If we run this code without a `hello.txt` file, we’ll see an error message from the `panic!` call that the `unwrap` method makes.

- Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier. The syntax of expect looks like this:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

- The error message used by `expect` in its call to `panic!` will be the parameter that we pass to `expect`.

## Propagating Errors

- When you’re writing a function whose implementation calls something that might fail, instead of handling the error within this function, you can return the error to the calling code so that it can decide what to do.

- The following code exerpt shows a function that reads a username from a file. If the file doesn’t exist or can’t be read, this function will return those errors to the code that called this function:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

- The following code provides the same functionality of the exerpt above that is using `match` expressions, however it's using the `?` operator to explicitly propagate the error, which makes the code much more concise:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

- This code can even be shortened by aggregating the `read_to_string()` function into `File::open`:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

- Reading a file into a string is a fairly common operation, so Rust provides the convenient `fs::read_to_string` function that opens the file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it:

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

- The main function can be changed to include a `Result<T, E>` return:

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

## To panic! or Not to panic!

- When you choose to return a `Result` value, you give the calling code options rather than making the decision for it. The calling code could choose to attempt to recover in a way that’s appropriate for its situation, or it could decide that an `Err` value in this case is unrecoverable, so it can call `panic!` and turn your recoverable error into an unrecoverable one.

- Returning `Result` is a good default choice when you’re defining a function that might fail.
