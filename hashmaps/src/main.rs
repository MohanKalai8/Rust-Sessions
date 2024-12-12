use std::collections::HashMap;

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing values in a Hash Map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!(" {team_name} : {score}");

    // hashmaps and ownerships
    let red_color = String::from("Red");
    scores.insert(red_color,20);  // now we can't use `red_color`
    // println!("{red_color}");

    // upating the hashmap
    scores.insert(String::from("Blue"), 100);

    // Adding a key and value only if key isn't present
    scores.entry(String::from("Black")).or_insert(30);

    for (key,value) in &scores {
        println!("{key}: {value}");
    }
}
