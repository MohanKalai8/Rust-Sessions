## Structs
- Normal structs
- Tuple structs
- Unit-Like structs


### Printing structs
- We have to use Debug trait on the struct definition
- We can print with `{:?}` or `{:#?}`
- `dbg!` macro prints to the standard error console stream(`stderr`), as opposed to `println!`, which prints to the standard output console stream

### Method Syntax
- Unlike functions, methods are defined within the context of a struct, their 1st parameter is `self` represents instance of the struct being called on

### Associated Functions
- All the functions defined within in the `impl` block are called associated functions because they're associated with the type name after the `impl`.
- Associated fn's that aren't methods are often used for constructors that will return a new instance of the struct. 