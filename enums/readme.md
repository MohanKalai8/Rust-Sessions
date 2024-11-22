## Enums

### Option Enums
he Option type encodes the very common scenario in which a value could be something or it could be nothing.
here T is generic we will talk about that in coming sessions
```rust
enum Option<T> {
    None,
    Some(T),
}
```

### The match Control Flow Construct
example : 

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### if let flow
Example : 
```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```
