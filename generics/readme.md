## Generics
We use generics to create definitions for items like function signatures or structs,while we can then use with many different concrete data types.

- Generics helps us to reduce code duplication.
- We can use generics on struct Definitions
- We can also use generics on Enum Definitions like `Option<T>` and `Result<T>` enum
   - Option:
   ```rust
    enum Option<T> {
        Some(T),
        None,
    }
    ```
    - Result Enum:
   ```rust
    enum Result<T,E>{
        Ok(T),
        Err(E),
    }
    ```
- Performace of code won't effect if we use generics.
