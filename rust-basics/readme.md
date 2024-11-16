# Rust Basics

## Variables and Mutability

- Rust variables are immutable by default
- If you want to mutate them just add `mut` keyword
- Constant variables can be declared using `const` keyword

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
### Shadowing
- While shadowing we can also change the type of the variable, where as for `mut` variables we can't change type

```rust
fn main() {
    let x = 5;
    let x = x + 1;
}
```

## Data Types 

### Scalar Types:

1. Integer Types:
   - Signed : i8, i16 i32, i64, i128, isize
   - Unsigned : u8, u16, u32, u64, u128, usize
  
2. Floating-Point Types:
   - f32, f64(default)
  
3. Boolean Type:
   - True, false
  
4. Character Type:
   - ex : `let c = 'z';`
  
### Compund Types: 
1. The Tuple Type: group values with variety of types
2. Array Type: group values of same type
   1. Unlike arrays in some other languages, arrays in Rust have a fixed length.

## Functions:
- main() function : entry point function
- remaining functions : `fn func_name()` 

### Statements and Expressions

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. Let’s look at some examples.


## Comments
- Comments can be declared with `//`

## Control Flow 
- if Expressions
- Loops : loop, while, for
  
