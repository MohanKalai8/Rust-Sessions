## The Slice Type
`slices` let you reference a contiguos sequence of elements in a collection rather than the whole collection. A `slice` is kind of a reference, so it does not have ownership.

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```
The above code will give an error, if we try to use the `word` after clearing `s`. Because we already gave immutable reference to `first_word()`, we can't again give mutable reference to `s.clear()`

### String slice
A string slice is a reference to part of s `String`
```rust
    let s = String::from("hello world");

    let hello = &s[0..5]; // hello 
    let world = &s[6..11]; // world
```

