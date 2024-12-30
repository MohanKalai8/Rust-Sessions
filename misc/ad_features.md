# Advanced Features in Rust

A quick reference guide to advanced Rust features with practical examples.

## Unsafe Rust

### What Can Be Done in Unsafe Blocks
```rust
unsafe {
    // 1. Dereference raw pointers
    // 2. Call unsafe functions
    // 3. Access/modify mutable static variables
    // 4. Implement unsafe traits
    // 5. Access fields of unions
}
```

### Practical Examples

#### 1. Raw Pointers
```rust
let mut num = 5;
let r1 = &num as *const i32;    // Immutable raw pointer
let r2 = &mut num as *mut i32;  // Mutable raw pointer

unsafe {
    println!("r1 is: {}", *r1);
    *r2 = 10;
}
```

#### 2. Unsafe Function
```rust
unsafe fn dangerous() {
    // Unsafe operations here
}

unsafe {
    dangerous();
}
```

#### 3. Static Mutable Variable
```rust
static mut COUNTER: u32 = 0;

unsafe {
    COUNTER += 1;
    println!("COUNTER: {}", COUNTER);
}
```

## Advanced Traits

### Associated Types with Example
```rust
trait Container {
    type Item;
    fn get(&self) -> Option<&Self::Item>;
}

struct Stack<T> {
    items: Vec<T>
}

impl<T> Container for Stack<T> {
    type Item = T;
    fn get(&self) -> Option<&T> {
        self.items.last()
    }
}
```

### Operator Overloading Example
```rust
use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

// Usage
let c1 = Complex { real: 1.0, imag: 2.0 };
let c2 = Complex { real: 3.0, imag: 4.0 };
let sum = c1 + c2;  // Complex { real: 4.0, imag: 6.0 }
```

### Supertraits Example
```rust
trait Drawable {
    fn draw(&self);
}

trait Button: Drawable {  // Button requires Drawable
    fn click(&self);
}

struct RoundButton {
    label: String,
}

impl Drawable for RoundButton {
    fn draw(&self) {
        println!("Drawing button with label: {}", self.label);
    }
}

impl Button for RoundButton {
    fn click(&self) {
        println!("Clicked: {}", self.label);
    }
}
```

## Advanced Types

### Type Aliases with Complex Example
```rust
// Complex type alias
type Result<T> = std::result::Result<T, std::io::Error>;

// Function using the alias
fn read_file() -> Result<String> {
    std::fs::read_to_string("file.txt")
}

// Callback type alias
type Callback = Box<dyn Fn(i32) -> i32>;

struct Calculator {
    operation: Callback,
}

impl Calculator {
    fn new(cb: Callback) -> Calculator {
        Calculator { operation: cb }
    }
}
```

### Never Type Example
```rust
fn get_user_input() -> ! {
    loop {
        println!("Enter command:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "quit" => std::process::exit(0),
            cmd => println!("Unknown command: {}", cmd),
        }
    }
}
```

## Advanced Functions and Closures

### Function Pointers with Context
```rust
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// Usage
let result = do_twice(add_one, 5);  // Returns 7
```

### Returning Closures Example
```rust
fn create_adder(amount: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + amount)
}

// Usage
let add_five = create_adder(5);
println!("Sum: {}", add_five(10));  // Prints: Sum: 15
```

## Macros

### Declarative Macro Example
```rust
#[macro_export]
macro_rules! hash_map {
    ($($key:expr => $value:expr),*) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

// Usage
let map = hash_map!{
    "one" => 1,
    "two" => 2,
    "three" => 3
};
```

### Procedural Macro Example
```rust
use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Implementation here
    "fn hello_macro() { println!(\"Hello, Macro!\"); }".parse().unwrap()
}

// Usage
#[derive(HelloMacro)]
struct Pancakes;
```

### Function-like Macro Example
```rust
#[macro_export]
macro_rules! sql_query {
    ($table:ident WHERE $($field:ident = $value:expr),*) => {{
        let mut query = format!("SELECT * FROM {} WHERE ", stringify!($table));
        let conditions = vec![$(format!("{} = '{}'", stringify!($field), $value)),*];
        query + &conditions.join(" AND ")
    }};
}

// Usage
let query = sql_query!(users WHERE name = "John", age = 30);
```

## Best Practices

1. Use unsafe code only when necessary and document why it's safe:
```rust
/// This unsafe block is safe because we guarantee that:
/// 1. The pointer is valid
/// 2. The data won't be modified elsewhere
unsafe {
    // unsafe code here
}
```

2. Prefer associated types over generic parameters when there's a 1:1 relationship
3. Use type aliases to simplify complex types
4. Document all unsafe code thoroughly
5. Test macros with different input patterns

## Common Gotchas and Solutions

1. Raw Pointer Safety:
```rust
// Wrong
let ptr = 0x12345 as *const i32;  // Dangerous!

// Right
let value = 42;
let ptr = &value as *const i32;
```

2. Trait Object Sizing:
```rust
// Wrong
struct Wrong {
    trait_object: dyn MyTrait,  // Error: doesn't have a size known at compile-time
}

// Right
struct Right {
    trait_object: Box<dyn MyTrait>,  // OK: Box has a known size
}
```

For more detailed information, refer to [The Rust Book - Advanced Features](https://doc.rust-lang.org/book/ch19-00-advanced-features.html)
