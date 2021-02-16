# Managing Growing Projects with Packages, Crates, and Modules

- Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the __module system__, include:

  - *Packages*: A Cargo feature that lets you build, test, and share crates
  - *Crates*: A tree of modules that produces a library or executable
  - *Modules* and *use*: Let you control the organization, scope, and privacy of paths
  - *Paths*: A way of naming an item, such as a struct, function, or module

## Packages and Crates

- *Crate*: A crate is a binary or library. The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.

- *Package*: A package is one or more crates that provide a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates.

  - If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

## Defining Modules to Control Scope and Privacy

- The `use` keyword brings a path into scope.
- The `pub` keyword make items public.

- __Modules__ let us organize code within a crate into groups for readability and easy reuse.

  - The module tree might remind you of the filesystem’s directory tree on your computer; this is a very apt comparison! Just like directories in a filesystem, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.

## Paths for Referring to an Item in the Module Tree

- To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. If we want to call a function, we need to know its path.

- Our preference is to specify absolute paths because it’s more likely to move code definitions and item calls independently of each other.

- Modules aren’t useful only for organizing your code. They also define Rust’s privacy boundary: the line that encapsulates the implementation details external code isn’t allowed to know about, call, or rely on. So, if you want to make an item like a function or struct private, you put it in a module.

## Making Structs and Enums Public

- If we use `pub` before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis.

```rust
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
```

- In the example above only the `toast` field is public, however if the `struct` has the `pub` keywork and none of the fields do, all fields are considered public.

- Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public.

```rust

#![allow(unused)]
fn main() {
  mod back_of_house {
      pub enum Appetizer {
          Soup,
          Salad,
      }
  }

  pub fn eat_at_restaurant() {
      let order1 = back_of_house::Appetizer::Soup;
      let order2 = back_of_house::Appetizer::Salad;
  }
}
```

## Bringing Paths into Scope with the use Keyword

- We can bring a path into a scope once and then call the items in that path as if they’re local items with the use keyword.

- We bring the `crate::front_of_house::hosting` module into the scope of the `eat_at_restaurant` function so we only have to specify `hosting::add_to_waitlist` to call the `add_to_waitlist` function in `eat_at_restaurant`:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem.

## Creating Idiomatic use Paths

- Bringing the function’s parent module into scope with `use` so we have to specify the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.

- On the other hand, when bringing in structs, enums, and other items with `use`, it’s idiomatic to specify the full path. Example of the idiomatic way to bring the standard library’s HashMap struct into the scope of a binary crate:

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

## Providing New Names with the as Keyword

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

- In the second use statement, we chose the new name `IoResult` for the `std::io::Result` type, which won’t conflict with the `Result` from `std::fmt` that we’ve also brought into scope.

## Re-exporting Names with pub use

- When we bring a name into scope with the `use` keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine `pub` and `use`. This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

## Using External Packages

- Members of the Rust community have made many packages available at crates.io, and pulling any of them into your package involves these same steps: listing them in your package’s `Cargo.toml` file and using `use` to bring items from their crates into scope.

- Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from crates.io and make rand available to our project.

```toml
[dependencies]
rand = "0.5.5"
```

## Using Nested Paths to Clean Up Large use Lists

- If we’re using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in our files.

- Regular `use`:

```rust
use rand::Rng;
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

- Nested `use`:

```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

- This line brings `std::io` and `std::io::Write` into scope:

```rust
use std::io::{self, Write};
```

## The Glob Operator

If we want to bring all public items defined in a path into scope, we can specify that path followed by `*`, the glob operator:

```rust
use std::collections::*;
```

## Separating Modules into Different Files

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- Using a semicolon after `mod front_of_house` rather than using a block tells Rust to load the contents of the module from another file with the same name as the module.
