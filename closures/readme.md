# Rust Closures

This project contains examples and explanations of closures in Rust. Closures are anonymous functions you can save in a variable or pass as arguments to other functions. They are similar to lambdas in other programming languages.

## Key Concepts

- **Syntax**: Closures are defined using the `|param| {}` syntax. For example:
    ```rust
    let add_one = |x: i32| -> i32 { x + 1 };
    let result = add_one(5); // result is 6
    ```
- **Type Inference**: Rust can infer the types of parameters and return values.
- **Capturing Variables**: Closures can capture variables from their environment.
- **Ownership and Borrowing**: Closures can take ownership, borrow immutably, or borrow mutably.

## Example of a Multi-line Closure without Explicit Input Types

```rust
fn main() {
        let num = 10;
        let multiply = |x| {
                let result = x * num;
                result
        };

        let result = multiply(5); // result is 50
        println!("The result is {}", result);
}
```