## Hashmaps
- HashMap<K, V> : stores a mapping of keys of type K to values of type V using a hashing function.
- All the keys must have same type and all values must have same type.
  
- Creation
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"),10);
scores.insert(String::from("Yellow"), 50);
```

- Accessing values in a Hash Map

```rust
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

- Updating a Hash Map
```rust
    scores.insert(String::from("Blue"), 25);
```

- Adding a Key and Value Only if a key isn't present

```rust
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Red")).or_insert(100);
```
