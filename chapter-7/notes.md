# Managing Growing Projects with Packages, Crates, and Modules

- Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the __module system__, include:

  - *Packages*: A Cargo feature that lets you build, test, and share crates
  - *Crates*: A tree of modules that produces a library or executable
  - *Modules* and use: Let you control the organization, scope, and privacy of paths
  - *Paths*: A way of naming an item, such as a struct, function, or module

## Packages and Crates

- *Crate*: A crate is a binary or library. The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.

- *Package*: A package is one or more crates that provide a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates.

  -  If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

