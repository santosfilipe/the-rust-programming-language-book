# Generic Types, Traits, and Lifetimes

- Generics are abstract stand-ins for concrete types or other properties. When we’re writing code, we can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.

- Similar to the way a function takes parameters with unknown values to run the same code on multiple concrete values, functions can take parameters of some generic type instead of a concrete type, like `i32` or `String`.

- Lifetimes allow us to borrow values in many situations while still enabling the compiler to check that the references are valid.

## Removing Duplication by Extracting a Function

- Duplicating code is not a practice found in proper production code. Whenever a behavior has to be reused, one can write a function that can then be called wherever the behavior is necessary. At times a function will be bound to a specific type, and therefore not generic enough.

- Generics allow code to operate on abstract types.

- For example, say we had two functions: one that finds the largest item in a slice of `i32` values and one that finds the largest item in a slice of `char` values. How would we eliminate that duplication?

## Generic Data Types

- We can use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

### In Function Definitions

- When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value. Doing so makes our code more flexible and provides more functionality to callers of our function while preventing code duplication.

- Considering this example of the need to write a function to find the largest number in a given slice, one would have to code a specific function for each concrete type to account for different slice types:

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

- Although the necessary logic is somewhat encapsulated into the functions, there is still code duplication due to the use of concrete types.

- A similar function could be written using generics to account for any given slice of any concrete type, therefore properly encapsulating the logic without duplicating any code:

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

## In Struct Definitions

- Structs can also be created using generics. The following struct is built to support any concrete type:

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.5, y: 4.7 };

    println!("Interger struct: x {}, y {}.", integer.x, integer.y);
    println!("Interger float: x {}, y {}.", float.x, float.y);
}
```

- The important aspect to consider when using generics with a `struct` is the correct use of the same type throughout out the struct's fields. Using different concrete types when calling the same struct will result in an error.

- To define a `struct` where x and y are both generics but could have different types, we can use multiple generic type parameters:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

## In Enum Definitions
