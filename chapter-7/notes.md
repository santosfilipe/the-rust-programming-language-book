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

-  Our preference is to specify absolute paths because it’s more likely to move code definitions and item calls independently of each other.

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

