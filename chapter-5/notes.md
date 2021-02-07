# Using Structs to Structure Related Data

- A struct is a custom data type that lets you name and package together multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.

## Defining and Instantiating Structs

- To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

- To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct and then add curly brackets containing `key: value` pairs:

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

- The order in which the fields of a struct are declared within an instance does not matter.

- To get a specific value from a struct, we can use dot notation. If we wanted just this user’s email address, we could use `user1.email` wherever we wanted to use this value.

- As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

## Using the Field Init Shorthand when Variables and Fields Have the Same Name

Because the parameter names and the struct field names are exactly the same, we can use the __field init__ shorthand syntax to rewrite `build_user` so that it behaves exactly the same but doesn’t have the repetition of `email` and `username`:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

## Creating Instances From Other Instances With Struct Update Syntax

- The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

## Using Tuple Structs without Named Fields to Create Different Types

- You can also define structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.

- To define a tuple struct, start with the `struct` keyword and the struct name followed by the types in the tuple. For example, here are definitions and usages of two tuple structs named `Color`:

```rust
struct Color(i32, i32, i32);
```

## Unit-Like Structs Without Any Fields

- You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type.

## Ownership of Struct Data

- It’s possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

- This code does not work because it lacks `lifetimes`:

```rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

```text
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:15
  |
2 |     username: &str,
  |               ^ expected lifetime parameter

error[E0106]: missing lifetime specifier
 --> src/main.rs:3:12
  |
3 |     email: &str,
  |            ^ expected lifetime parameter
```

## An Example Program Using Structs

- The code for this section is structured in 3 different directories named `/rectangles-*`. 

- The `println!` macro does not work out-of-the-box for `structs`. The following code will not work:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

- The `println!` macro has to be adjusted to include debug output formatting:

```rust
println!("rect1 is {:?}", rect1);
```

- To enable printing of `structs` using this identifier, the `struct` has to be explicitly enabled to:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

- The `#[derive(Debug)]` annotation is used for this matter.

## Method Syntax

- Methods are different from functions in that they’re defined within the context of a `struct`, `enum` or a `trait` object, and their first parameter is always `self`, which represents the instance of the `struct` the method is being called on.

- Methods can take multiple parameters that we add to the signature after the self parameter, and those parameters work just like parameters in functions.

## Associated Functions

- Another useful feature of `impl` blocks is that we’re allowed to define functions within `impl` blocks that don’t take `self` as a parameter. These are called associated functions because they’re associated with the struct.

- Associated functions are often used for constructors that will return a new instance of the struct.
