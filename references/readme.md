## References and Borrowing

- A `reference` is like a pointer in that it's an address we can follow to access the data stored at that address.
- We can pass reference with this `ampersands` `&`, Also known as Borrowing
- This reference allow you to refer to some value without taking ownership
- The opposite of `referencing` by using the `&` is `dereferencing`, which is accomplished with the dereference operator `*`
- Mutable and Immutable references.
  
### Mutable References

```rust 
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
For mutable References:
- We change `s` to be `mut`
- Then we create a mutable reference with `&mut s`
- The change function accepts a mutable reference with `some_string: &mut String` 
- If you have a mutable reference to a value, you can have no other references to that value.
  - This will give error,as we can't borrow `s` as mutable more than once at a time.
  ```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
  ```


### Dangling References

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```
This function's return type contains a borrowed value, but there is no value for it to be borrowed from

```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

## The Rules of References
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.