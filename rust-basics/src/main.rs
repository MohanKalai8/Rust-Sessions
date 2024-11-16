fn main() {
    println!("Main function");
    let x = do_math(5,10);
    println!("The value of x {x}");

    ///////////// Loops /////////////
    for number in 1..4 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn do_math(x:i32,y:i32) -> i32{
    println!("Math function");
    x+y
}
