## Packages and Crates
- `Packages:` A cargo feature that lets you build, test, and share crates
- `Crates:` A tree of modules that produces a library or executable
- `Modules` and `use:` Let you control the organization, scope, and privacy of paths
- `Paths:` A way of naming an item, such as a struct, function, or module


- A crate is the smallest amount of code that the Rust compiler considers at a time
- A package is a bundle of one or more crates that provides a set of functionality

## Modules Cheat Sheet
- Start from the crate root
- Declaring modules - `garden` looks in below places
  - Inline : `mod garden`
  - In the file src/garden.rs
  - In the file src/garden/mod.rs
- Declaring submodules
- Paths to code in modules
- Private vs public 
- The `use` keyword

## Paths for Referring to an item in the Module Tree
A path can take two forms:
- `absolute path` : is the full path starting from a crate root
- `relativef path` : starts from the current module and uses `self`,  `super`

- In Rust, all items (functions, methods,structs,enums,modules, and constants) are private to parent modules by default.

## Bringing Paths into Scope with the `use` keyword
- Creating idiomatic use paths
- Providing new names with the `as` keyword
- Re-exporting names with pub `use`
- Using external packages
- Using nested paths to clean up large use lists
- The Glob Operator

## Separating modules into different files


To install cargo modules crate
```rust
cargo install cargo-modules
```

To view tree structure
```rust
cargo modules structure --lib
```