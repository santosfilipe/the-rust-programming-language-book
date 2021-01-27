## Notes for Chapter 3

- [Rust reserved keywords.](https://doc.rust-lang.org/book/appendix-01-keywords.html)

### Variables and Mutability

- As mentioned in Chapter 2, **by default variables are immutable**.

- Constants aren’t just immutable by default—they’re always immutable.

- You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value must be annotated.

- Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

- The last difference is that constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

Example of a Rust constant: `const MAX_POINTS: u32 = 100_000;`

### Data Types

- Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.

- Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

