# Notes for Chapter 3

- [Rust reserved keywords.](https://doc.rust-lang.org/book/appendix-01-keywords.html)

## Variables and Mutability

- As mentioned in Chapter 2, **by default variables are immutable**.

- Constants aren’t just immutable by default—they’re always immutable.

- You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value must be annotated.

- Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

- The last difference is that constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

Example of a Rust constant: `const MAX_POINTS: u32 = 100_000;`

## Data Types

- Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.

- Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

### Integer

- Signed integer types start with `i`, instead of `u`. Each variant can be either signed or unsigned and has an explicit size. Signed and unsigned refer to whether it’s possible for the number to be negative.

### Floating-point

- Rust’s floating-point types are `f32` and `f64`. Default type is `f64` because on modern CPUs. The `f32` type is a single-precision float, and `f64` has double precision.

### Boolean

- As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using `bool`.

### Character

- Rust’s `char` type is the language’s most primitive alphabetic type, and the following code shows one way to use it. (Note that `char` literals are specified with single quotes, as opposed to string literals, which use double quotes.)

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

### Compound Types

- Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### Tuple

- Tuples have a fixed length: once declared, they cannot grow or shrink in size. We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

- To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

- In addition to destructuring through pattern matching, we can access a tuple element directly by using a period (`.`) followed by the index of the value we want to access. For example:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

### Array

- Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.

- In Rust, the values going into an array are written as a comma-separated list inside square brackets:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

- If you’re unsure whether to use an array or a vector, you should probably use a vector.

- An example of when you might want to use an array rather than a vector is in a program that needs to know the names of the months of the year. It’s very unlikely that such a program will need to add or remove months, so you can use an array because you know it will always contain 12 elements.

- You would write an array’s type by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array, like so:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

- An array is a single chunk of memory allocated on the stack. You can access elements of an array using indexing, like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```
