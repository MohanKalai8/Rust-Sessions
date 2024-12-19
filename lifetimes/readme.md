## Generic lifetime parameters
Lifetimes in Rust are a way of ensuring that references are valid as long as they are needed, preventing dangling references. They are denoted using a syntax like `'a` and are used to specify how long a reference should be valid. Lifetimes are particularly important in functions and structs to ensure memory safety without needing a garbage collector. Rust's borrow checker uses lifetimes to enforce these rules at compile time, making sure that references do not outlive the data they point to.

## Three rules of lifetimes
The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.

1. The first rule is, if a function with one parameter gets one lifeteime parameter: `fn:foo<'a>(x:&'a i32);` a function with two parameters gets two separte lifeime parameters `fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32` ; and so on.
2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
3. The third rule is that, if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters. 

If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error.

```rust
fn first_word(s: &str) -> &str { // 1
```

```rust
fn first_word<'a>(s: &'a str) -> &'a str { // 1 & 1(rules)
```

```rust
fn longest<'a, 'b>(x: &'a str, &'b str) -> &str { // gives error,as we we can't figure out return type lifetime
```

## static lifetimes
- static lifetimes can live for the entire duration of the program.
- All string literals have the `'static` lifetime
    ```rust
    let s: &'static str = "I have a static lifetime.";
    ```



