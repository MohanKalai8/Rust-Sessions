## Common Collections
- A `vector` allows you to store a variable number of values next to each other.
- A `string` is a collection of characters. 
- A `hash map` allows you to associate a value with a specific key

### Vector
- allows you to store more than one value in a single data structure. vectors can only store values of the same type. It is dynamically sized

```rust
let v: Vec<i32> = Vec::new();
```
rust provides a `vec!` macro, which will create a new vector that holds the values you give it. `i32` is the default integer type

```rust
let v = vec![1, 2, 3];
```
#### Updating a vector
- by using `push` method we can add elements to it

#### Reading Elements of Vectors
- There are two ways to reference a value stored in a vector: via indexing or by using the `get` method.
- Using `&` and `[]` gives us a reference to the element at the index value. When we use `get` method with the index passes as an argument, we get an `Option<&T>` that we can use with `match`. 

#### Using Enums to Store Multiple Values
- when we need one type to represent elements of different types, we can define and use an enum!
```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```