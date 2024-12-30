# Patterns and Matching in Rust

A quick reference guide to pattern matching concepts in Rust.

## Pattern Syntax

### Matching Literals
```rust
match x {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("anything"),
}
```

### Matching Named Variables
```rust
let x = Some(5);
match x {
    Some(value) => println!("Got value: {}", value),
    None => println!("Got nothing"),
}
```

### Multiple Patterns
```rust
match x {
    1 | 2 => println!("one or two"),
    3..=5 => println!("three through five"),
    _ => println!("something else"),
}
```

## Pattern Types

### 1. Refutable vs Irrefutable 
- **Irrefutable**: Patterns that match any value (a pattern that will always match)
  ```rust
  let x = 5; // x is irrefutable
  ```
- **Refutable**: Patterns that might fail to match
  ```rust
  if let Some(x) = optional { // Some(x) is refutable
      println!("got a value: {}", x);
  }
  ```

### 2. Pattern Matching Guards
```rust
match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

## Common Pattern Uses

### 1. Destructuring

#### Structs
```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };
let Point { x, y } = p;
```

#### Tuples
```rust
let tuple = (1, 2, 3);
let (x, y, z) = tuple;
```

#### Enums
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to {}, {}", x, y),
    Message::Write(text) => println!("Text message: {}", text),
}
```

### 2. Ignoring Values

```rust
// Ignore entire value
let _ = expression();

// Ignore parts of value
let (x, _, z) = tuple;

// Ignore remaining parts
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let Point { x, .. } = point;
```

## Match Expressions

### 1. Basic Match
```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_expression,
}
```

### 2. If Let
```rust
// Instead of
match optional {
    Some(x) => println!("{}", x),
    None => (),
}

// You can use
if let Some(x) = optional {
    println!("{}", x);
}
```

### 3. While Let
```rust
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

## Best Practices

1. Use `match` when you want to handle all cases exhaustively
2. Use `if let` for simpler single-pattern matching
3. Use destructuring to access struct/enum fields directly
4. Use pattern guards for complex conditions
5. Remember to handle all cases or use `_` wildcard
6. Use `@` bindings to test and capture values
   ```rust
   match num {
       n @ 1..=5 => println!("Got a range number: {}", n),
       n => println!("Something else: {}", n),
   }
   ```

## Common Gotchas

1. Match arms must be exhaustive
2. Pattern matching is sequential (first match wins)
3. `if let` doesn't enforce exhaustive checking
4. Reference patterns need explicit `&`
5. Cannot use `if let` with multiple patterns (use `match` instead)

For more detailed information, refer to [The Rust Book - Patterns and Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
