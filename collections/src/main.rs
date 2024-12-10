#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row:Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("The final vector is {:#?}", row);

    match row.get(0){
        Some(SpreadsheetCell::Int(value)) => println!("The value is int {value}"),
        Some(_) => println!("This is some other value"),
        None => println!("No value exists")
    }
}
