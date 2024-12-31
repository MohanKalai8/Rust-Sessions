# Rust Web Server Implementation

A simple web server built following Chapter 20 of the Rust Book, demonstrating the progression from a single-threaded to a multi-threaded server with graceful shutdown.

## Implementation Stages

### 1. Single-Threaded Server
- Created basic TCP listener on localhost:7878
- Handled incoming connections sequentially
- Implemented basic request parsing and response generation
- Served static HTML files (index.html and 404.html)
- Added simulated slow response (/sleep) to demonstrate blocking

```rust
// Basic single-threaded handling
for stream in listener.incoming() {
    handle_connection(stream.unwrap());
}
```

### 2. Multi-threaded Server
- Implemented `ThreadPool` to manage multiple threads
- Created worker threads to handle connections concurrently
- Used channels (mpsc) for communication between threads
- Implemented thread-safe job queue using Arc and Mutex

```rust
// Multi-threaded handling with thread pool
let pool = ThreadPool::new(4);
for stream in listener.incoming() {
    pool.execute(|| {
        handle_connection(stream.unwrap());
    });
}
```

### 3. Graceful Shutdown
- Added message passing system between threads
- Implemented `Drop` trait for cleanup
- Created two types of messages:
  - `NewJob`: For handling new connections
  - `Terminate`: For shutting down workers
- Added proper worker thread cleanup

## Key Components

1. **ThreadPool**: Manages a pool of worker threads
2. **Worker**: Handles job execution in separate threads
3. **Message System**: Enum for communication
   ```rust
   enum Message {
       NewJob(Job),
       Terminate,
   }
   ```
4. **Channel Communication**: Using `mpsc` for thread communication
5. **Thread Synchronization**: Using `Arc<Mutex<>>` for shared state

## Usage

1. Start the server:
   ```bash
   cargo run
   ```
2. Access endpoints:
   - Main page: `http://127.0.0.1:7878/`
   - Slow response: `http://127.0.0.1:7878/sleep`


