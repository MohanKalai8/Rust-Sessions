# Iterators in Rust

Iterators are a powerful feature in Rust that allow you to process sequences of elements in a functional style. They provide a way to iterate over collections such as arrays, vectors, and other data structures.

## Key Concepts

- **Iterator Trait**: The core trait that defines an iterator. It has a single required method, `next`, which returns an `Option` with the next item or `None` when iteration is complete.
- **Creating Iterators**: You can create custom iterators by implementing the `Iterator` trait for your types.
- **Adaptor Methods**: Methods like `map`, `filter`, and `collect` that allow you to transform and consume iterators in various ways.
- **Lazy Evaluation**: Iterators are lazy, meaning they do not perform any computation until they are consumed.

