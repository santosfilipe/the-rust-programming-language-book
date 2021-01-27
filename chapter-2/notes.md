## Notes for Chapter 2

- In Rust, variables are immutable by default.

`let foo = 5;` // immutable

`let mut bar = 5;` // mutable

- The `::` syntax in the `::new` line indicates that `new` is an associated function of the `String` type. An associated function is implemented on a type, in this case `String`, rather than on a particular instance of a String. Some languages call this a static method.

- Another neat feature of Cargo is that you can run the `cargo doc --open` command, which will build documentation provided by all of your dependencies locally and open it in your browser. If you’re interested in other functionality in the `rand` crate, for example, run `cargo doc --open` and click `rand` in the sidebar on the left.

- Rust has a strong, static type system.

- Shadowing lets us reuse the `guess` variable name rather than forcing us to create two unique variables, such as `guess_str` and `guess` for example. => A fair feature to 'solve' the type casting hell.
