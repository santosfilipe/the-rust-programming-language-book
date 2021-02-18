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

- The following code can be used to iterate through a vactor:

```rust
let v = vec![100, 32, 57];

for i in &v {
    println!("{}", i);
}
```

- We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements. To change the value that the mutable reference refers to, we have to use the dereference operator (`*`) to get to the value in `i` before we can use the `+=` operator:

```rust
let mut v = vec![100, 32, 57];

for i in &mut v {
    *i += 50;
}
```

### Using an Enum to Store Multiple Types

- When we need to store elements of a different type in a vector, we can define and use an enum. We can define an enum whose variants will hold the different value types, and then all the enum variants will be considered the same type: that of the enum. Then we can create a vector that holds that enum and so, ultimately, holds different types:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

## Storing UTF-8 Encoded Text with Strings

- It’s useful to discuss strings in the context of collections because strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.

### Creating a New String

```rust
let mut s = String::new();
```

- After create a string, one can insert data into it:

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

- We can also use the function String::from to create a String from a string literal:

```rust
let s = String::from("initial contents");
```

### Updating a String

- You can conveniently use the `+` operator or the `format!` macro to concatenate `String` values:

- We can grow a `String` by using the `push_str` method to append a string slice:

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

### Concatenation with the + Operator or the format! Macro

- Often, you’ll want to combine two existing strings. One way is to use the `+` operator:

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

- If we need to concatenate multiple strings, the behavior of the `+` operator gets unwieldy:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

- For more complicated string combining, we can use the `format!` macro:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

### Indexing into Strings

- Rust strings don’t support indexing.

### Slicing Strings

- To be more specific in your indexing and indicate that you want a string slice, rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

### Methods for Iterating Over Strings

- If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the `chars` method:

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

- But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.

## Storing Keys with Associated Values in Hash Maps

- The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`.

### Creating a New Hash Map

- You can create an empty hash map with `new` and add elements with `insert`:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

- Hash maps also have less support from the standard library; there’s no built-in macro to construct them, for example.

- Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

- The `zip` method can be used to combine 2 `vectors` into a `HashMap`:

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
```

- The type annotation `HashMap<_, _>` is needed here because it’s possible to collect into many different data structures and Rust doesn’t know which you want unless you specify. For the parameters for the key and value types, however, we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors.

### Hash Maps and Ownership

- For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values.

### Accessing Values in a Hash Map

- We can get a value out of the hash map by providing its key to the `get` method:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

- We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a `for` loop:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### Updating a Hash Map

- When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned. You could replace the old value with the new value, completely disregarding the old value. You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value. Or you could combine the old value and the new value.

#### Overwriting a Value

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

#### Only Inserting a Value If the Key Has No Value

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

- The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value. This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.

#### Updating a Value Based on the Old Value

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

- The `or_insert` method actually returns a mutable reference `(&mut V)` to the value for this key. Here we store that mutable reference in the `count` variable, so in order to assign to that value, we must first dereference `count` using the asterisk `(*)`.

### Hashing Functions

- By default, `HashMap` uses a “cryptographically strong” hashing function that can provide resistance to Denial of Service (DoS) attacks. 