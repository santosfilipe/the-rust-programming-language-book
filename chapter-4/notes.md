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

## Ways Variables and Data Interact: **Clone**

- If we do want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

- The `clone` method is used with data types such as `String` due to the unpredictability of the size of data, which leads strings to be stored on the heap. Integers are stored on the stack due to it's fixed size, therefore there is no need to `clone` an integer and one can easily do it like in most other programming languages:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

## Ownership and Functions

- The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

## References and Borrowing

- Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

- Note that we pass `&s1` into `calculate_length` and, in its definition, we take `&String` rather than `String`. These ampersands are references, and they allow you to refer to some value without taking ownership of it.

- We call having references as function parameters borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.

- Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

## Mutable References

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- First, we had to change `s` to be `mut`. Then we had to create a mutable reference with `&mut s` and accept a mutable reference with `some_string: &mut String`.

## The Slice Type

- Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

- A `String` can be converted into an array of bytes:

```rust
let bytes = s.as_bytes();
```

### String Slices

- A string slice is a reference to part of a String, and it looks like this:

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
```

- We can create slices using a range within brackets by specifying `[starting_index..ending_index]`, where `starting_index` is the first position in the slice and `ending_index` is one more than the last position in the slice.

- The type that signifies “string slice” is written as `&str`.

- The correct implementation for a function that receives a `String` and has to return it's first word, is the following:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

- A more experienced Rustacean would write the signature shown belown instead because it allows us to use the same function on both `&String` values and `&str` values:

```rust
fn first_word(s: &str) -> &str {

}
```

- If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the entire String. Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality.

## Other Slices

- Consider this array:

```rust
let a = [1, 2, 3, 4, 5];
```

- Just as we might want to refer to a part of a string, we might want to refer to part of an array. We’d do so like this:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```