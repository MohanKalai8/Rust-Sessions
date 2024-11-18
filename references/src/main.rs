fn main() {
    let mut s = String::from("hello");

    let len = get_len(&s); // immutable reference
    println!("The value of s is {s}");
    change(&mut s); // mutable reference

    println!("The length of the string is {len}");

    println!("The value of s is {s}");

    let string = no_dangle(); // getting ownership from no_dangle function
    println!("The value of string is {string}");
}

fn get_len(some_string: &String)->usize{
    some_string.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}