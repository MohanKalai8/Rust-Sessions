## Ownership
- Ownership is a set of rules that governs how Rust program manages memory

### Stack
- All data stored on the `stack` must have a known, fixed size.

### Heap
- Data with an unknown size at compile time or a size that might change must be stored on the `heap` instead.This process is called `allocating` on the heap
- While allocating space in heap, the heap returns a pointer which is the address of the location. 
- The pointer to the heap is a known, fixed size, you can store the pointer on the stack.
- pushing to the stack is faster than allocating on the heap

## Ownership Rules
1. Each value in Rust has an owner
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
   
## The string Type
This below struct will be stored in heap. we can push data to this string type
```
let s = String::from("hello");
```
### Variables and Data Interacting with Move
```rust
let x = 5;
let y = x; // x value will be copied to y, as x is stored in stack. As its size is known before compile time
```
Where as here,

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 value will be moved to s2. Since s1 is stored in heap. There should be only one owner.So ownership passed to s2
```

## Variables and Data Interacting with Clone
If we do want to deeply copy the heap data of the `String`. We can use a method called `clone`

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2});
```

In the above case, s1, s2 have two separate pointers with the same data in the heap.. 