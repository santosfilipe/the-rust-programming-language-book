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
