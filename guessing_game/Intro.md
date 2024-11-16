To check version : $ rust —version

To update rust : $ rustup update

To uninstall rust : $ rustup self uninstall

Local documentation : $ rustup doc

```
fn main() {

}
```

These lines define a function named main. The main function is special: it is always the first code that runs in every executable Rust program

`rustfmt {file_name}` command will help to automatically formats the projects

The body of the main function holds the following code:

```
    println!("Hello, world!");
```

There are four important details to notice here.
1. Rust style is to indent with four spaces, not a tab.
2. println! calls a Rust macro, (macro will have pre set of rules)
3. you see the "Hello, world!" string
4. we end the line with a semicolon (;)

## Compiling and Running Are Separate Steps

For compiling run the below command
```
$ rustc main.rs
```
After compiling successfully, Rust outputs a binary executable.

To run the main file

```
$ ./main
```

## Cargo

Cargo is Rust's build system and package manager. It will be helpful while working on larger projects to download external dependencies
 
Cargo comes installed with Rust if we used the official installers. To check version

```
cargo --version
```

## Creating a new project with cargo
```
$ cargo new hello_cargo
$ cd hello_cargo
```

`toml` tom's obvious minimal language
In Rust, packages of code are referred to as `crates`

## Building and Running a Cargo Project
```
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```
To run the executable file

```
$ ./target/debug/hello_cargo 
Hello, world!
```

We just built a project with cargo build and ran it with ./target/debug/hello_cargo, but we can also use cargo run to compile the code and then run the resultant executable all in one command:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo also provides a command called `cargo check`. It checks whether our source code is correct or not without executing

```
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```