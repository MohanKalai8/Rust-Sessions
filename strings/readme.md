## Strings
- Both `String` and string slices are UTF-8 encoded .
- Updating a String
    - we can use `+` operator, or `format!` macro to concatenate String values.
    - append to a `String` with `push_str`. It takes a string slice, so no ownership taken
    - the `push` method takes a single character as a parameter and adds it to the String.
- A `String` is a wrapper over a `Vec<u8>`
- The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes
- `&str` is a promise that the byte sequence it points to will always be valid UTF-8. Therefore a programmer who wants to e.g. print out an `&str` never needs to check if it is valid, or worry about accidentally interpreting an invalid string.

### Creating a New String
- String is implemented as wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
  
```rust
let mut s = String::new();

let data = "initial contents";
let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

we can also use the function `String::from` to create a `String` which is same as above which we did with `to_string()` method

```rust
let s = String::from("initial contents");
```

### Updating a String
- We can grow a `String` by using the `push_str` method to append a strig slice
  
```rust
let mut s = String::from("foo");
s.push_str("bar");
```

- The `push` method takes a single character as a parameter and adds it to the `String`

```rust
let mut s = String::from("lo");
s.push('l');
```

### Concatenation with the + Operator 
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```
- The `+` operator uses the `add` method, whose signature looks something like this:
  
```rust
fn add(self, s: &str) -> String {
```

### `format!` macro
- `format!` macro uses references, so it doesn't take ownershihp of any of its parameters


```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}"); // it returns a `String`
```

### Methods for Iterating over strings

- `chars()`
```rust
for c in "Зд".chars() {
    println!("{c}");
}
```
It will print

```rust
З
д
```

- `bytes()`
```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```
It will print

```rust
208
151
208
180
```