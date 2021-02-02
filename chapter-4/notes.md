# Ownership

- Both the stack and the heap are parts of memory that are available to your code to use at runtime, but they are structured in different ways.

- The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. Adding data is called pushing onto the stack, and removing data is called popping off the stack.

- The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating.  

- Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory.

## Ownership Rules

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Variable Scope

- A scope is the range within a program for which an item is valid.

## The String Type

- Not every string value can be known when we write our code: for example, what if we want to take user input and store it? For these situations, Rust has a second string type, `String`.

- This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a `String` from a string literal using the `from` function, like so:

```rust
let s = String::from("hello");
```

- With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
  - The memory must be requested from the memory allocator at runtime.
  - We need a way of returning this memory to the allocator when we’re done with our `String`.

- Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

## Ways Variables and Data Interact: **Move**

- Data types such as `String` will not be copied from one variable to another, once a second variable is assigned to to another holding a `String`, the data is moved from variable #1 to variable #2. Data is not copied from the heap, variable #2 will receive the pointer to the data on the heap, and from now on only variable #2 is valid.

- This code does not work, since data was moved from `s1` to `s2`:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```

- This code **works**, since data was moved from `s1` to `s2` and only `s2` is being used:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);
}
```
