# Rust Concurrency Concepts

## Threads
- **Definition**: Threads are the smallest unit of execution within a program.
- **Usage**: In Rust, threads can be imported from `std::thread` module.
- **Example**:
    ```rust
    use std::thread;

    let handle = thread::spawn(|| {
            // Code to run in the new thread
    });

    handle.join().unwrap();
    ```

## Message Passing
- **Definition**: Message passing is a way to transfer data between threads safely.
- **Usage**: Rust provides channels (`std::sync::mpsc`) for message passing.
- **Example**:
    ```rust
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
            tx.send("Hello from thread").unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    ```

## Shared State Concurrency
- **Definition**: Shared state concurrency involves multiple threads accessing shared data.
- **Usage**: Rust uses `Mutex` and `Arc` (Atomic Reference Counting) to manage shared state.
- **Example**:
    ```rust
    use std::sync::{Arc, Mutex};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
            });
            handles.push(handle);
    }

    for handle in handles {
            handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    ```

These concepts are fundamental to achieving safe and efficient concurrency in Rust.