## Writing Automated Tests
- `cargo test` command to run the tests
- To separate two types of arguments, we use the `--` operator

To run a specific test function
- `cargo test $function_name`

To only run the ignored tests
- `cargo test -- --ignored`

if we want to run all tests whether they're ignored or not
- `cargo test -- --include-ignored`

To run specific integration test
- `cargo test --test integration_test`
This above command runs only the tests in the `tests/integration_test.rs` file.
