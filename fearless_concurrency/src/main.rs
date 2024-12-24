use fearless_concurrency::thread;
use fearless_concurrency::messaging;
use fearless_concurrency::share_state;

fn main() {
    println!("****** Threads ******");
    thread::main();
    println!("\n****** Messaging Passing b/w threads ******");
    messaging::main();
    println!("\n****** Share State ******");
    share_state::main();
}
