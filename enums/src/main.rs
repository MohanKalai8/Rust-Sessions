use std::io;

enum MohanOrdering{
    AIsGreater,
    AIsLesser,
    AIsEqual
}

fn compare(a:u32, b:u32) -> MohanOrdering{
    if a > b {
        MohanOrdering::AIsGreater
    }
    else if a < b {
        MohanOrdering::AIsLesser
    }
    else {
        MohanOrdering::AIsEqual
    }
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    println!("Enter the value of a:");
    io::stdin().read_line(&mut a).expect("read line failed");
    println!("Enter the value of b:");
    io::stdin().read_line(&mut b).expect("read line failed");
    let a:u32 = a.trim().parse().expect("only numbers allowed");
    let b:u32 = b.trim().parse().expect("only numbers allowed");

    match compare(a,b){
        MohanOrdering::AIsGreater => {
            println!("a is greater : {a},{b}")
        }
        MohanOrdering::AIsLesser => {
            println!("a is lesser: {a},{b}");
        }
        MohanOrdering::AIsEqual => {
            println!("a and b are equal: {a},{b}");
        }
    }
}
