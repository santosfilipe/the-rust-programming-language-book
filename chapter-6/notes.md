# Enums and Pattern Matching

- Enums allow you to define a type by enumerating its possible variants.

## Defining an Enum

- Enum used to define the possible IP versions:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

- `IpAddrKind` is now a custom data type that we can use elsewhere in our code.

## Enum Values

- Enum instances can be created like this:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

-  It's possible, for instance, to define a function that takes any IpAddrKind:

```rust
fn route(ip_kind: IpAddrKind) {}
```

- Using enums has even more advantages. Thinking more about our IP address type, at the moment we don’t have a way to store the actual IP address data; we only know what kind it is. Structs will help:

```rust
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

- We can represent the same concept in a more concise way using just an enum, rather than an enum inside a struct, by putting data directly into each enum variant:

```rust
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
```

- Each variant can have different types and amounts of associated data:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

- You can put any kind of data inside an enum variant: strings, numeric types, structs, and even other enums for example.

- There is one more similarity between enums and structs: just as we’re able to define methods on structs using `impl`, we’re also able to define methods on enums:

```rust
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```

## The Option Enum and Its Advantages Over Null Values

- The Option type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing.

- Rust doesn’t have the null feature that many other languages have. __Null__ is a value that means there is no value there.

- Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

- The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter. `<T>` means the Some variant of the Option enum can hold one piece of data of any type.

- If we use None rather than `Some`, we need to tell Rust what type of `Option<T>` we have, because the compiler can’t infer the type that the `Some` variant will hold by looking only at a `None` value.

- In order to use an `Option<T>` value, you want to have code that will handle each variant. You want some code that will run only when you have a `Some(T)` value, and this code is allowed to use the inner `T`. You want some other code to run if you have a `None` value, and that code doesn’t have a `T` value available. The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

## The match Control Flow Operator

- Rust has an extremely powerful control flow operator called `match` that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Patterns that Bind to Values

- Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.

```rust
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

## Matching with Option<T>

- We can also handle `Option<T>` using match as we did with the Coin enum! Instead of comparing coins, we’ll compare the variants of `Option<T>`, but the way that the match expression works remains the same.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

## The _ Placeholder

- Rust also has a pattern we can use when we don’t want to list all possible values. For example, a u8 can have valid values of 0 through 255. If we only care about the values 1, 3, 5, and 7, we don’t want to have to list the others:

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

- This is a situation where a `if let` expression would be more suited.

## Concise Control Flow with if let

- The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest.

- Considering a scenario where we only care about the value `3`, with a `match` expression, the following code would be used:

```rust
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

- `if let` enables us to write the same functionality in a more concise form:

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```

- The syntax `if let` takes a pattern and an expression separated by an equal sign. It works the same way as a `match`, where the expression is given to the `match` and the pattern is its first arm.