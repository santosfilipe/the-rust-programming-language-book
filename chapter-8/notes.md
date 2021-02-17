# Common Collections

- Rust’s standard library includes a number of very useful data structures called collections. Most other data types represent one specific value, but collections can contain multiple values.

  - A `vector` allows you to store a variable number of values next to each other.
  - A `string` is a collection of characters.
  - A `hash map` allows you to associate a value with a particular key.

## Storing Lists of Values with Vectors

- Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
- Vectors can only store values of the same type.
- They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

### Creating a New Vector

```rust
let v: Vec<i32> = Vec::new();
```

- The Vec<T> type provided by the standard library can hold any type, and when a specific vector holds a specific type, the type is specified within angle brackets.

- In more realistic code, Rust can often infer the type of value you want to store once you insert values, so you rarely need to do this type annotation.

- Rust provides the vec! macro for convenience. The macro will create a new vector that holds the values you give it.

```rust
let v = vec![1, 2, 3];
```

### Updating a Vector

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

- Like any other struct, a vector is freed when it goes out of scope. When the vector gets dropped, all of its contents are also dropped, meaning those integers it holds will be cleaned up.

### Reading Elements of Vectors

- There are two ways to reference a value stored in a vector. In the examples, we’ve annotated the types of the values that are returned from these functions for extra clarity.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

### Iterating over the Values in a Vector

