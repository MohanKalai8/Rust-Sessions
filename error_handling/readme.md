## Error Handling

Rust groups errors into two major categories:
- Recoverable (`Result<T, E>`)
- Unrecoverable errors (`panic!`)

### Unrecoverable Errors with `panic!`
- There are two ways to cause a panic in practice:
  - By taking an action that causes our code to panic(such as accessing an array past the end)
  - By explicitly calling the `panic!`
  
### Recoverable Errors with Result
- `Result` enum is defined as having two variants, `Ok` and `Err`
```rust
enum Result<T, E>{
  Ok(T),
  Err(E),
}
```
- `T` is type of the value that will be returned in a success
- `E` represents the type of the error that will be returned in a failure case