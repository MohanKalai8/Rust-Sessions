# Minigrep Project

This project is a simple implementation of the `grep` command-line tool, built in Rust. It allows you to search for a string within a file and returns the lines that contain the string.

## Features

- **Case-sensitive search**: By default, the search is case-sensitive.
- **Case-insensitive search**: You can enable case-insensitive search by setting the `CASE_INSENSITIVE` environment variable. by running this command in CLI `export CASE_INSENSITIVE=true`
- **Error handling**: The program handles various errors gracefully, such as file not found or invalid input.

## Usage

To run the program, use the following command:

```sh
cargo run <query> <filename>
```

- `<query>`: The string you want to search for.
- `<filename>`: The file in which to search.

Example:

```sh
cargo run to poem.txt
```

## Code Structure

- **main.rs**: The entry point of the application. It handles argument parsing and calls the search functionality.
- **lib.rs**: Contains the core functionality, including the search logic and error handling.

## Environment Variables

- `CASE_INSENSITIVE`: Set this variable to enable case-insensitive search.

Example:
1. To set environment variable
```sh
export CASE_INSENSITIVE=true
```
2. To unset environment variable
```sh
unset CASE_INSENSITIVE
```

To redirect ouput to `output.txt` file
```sh
cargo run <query> <filename> > <filename>
cargo run to poem.txt > output.txt
```

## Testing

The project includes unit tests to ensure the search functionality works correctly. Run the tests using:

```sh
cargo test
```

This project is a great way to learn Rust and understand how to build command-line applications with error handling and testing.
