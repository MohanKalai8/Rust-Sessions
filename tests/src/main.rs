fn main() {
    println!("Hello, world!");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess {value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_adds_two_1() {
        let result = add_two(3);
        assert_ne!(result, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Mohan");
        assert!(
            result.contains("Mohan"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(200);
    }

    // using `Result<T, E> in Tests
    #[test]
    fn testing_result() -> Result<(), String> {
        let result = add(2,2);

        if result == 4 {
            Ok(())
        }else{
            Err(String::from("two plus two does not equal four"))
        }
    }
}
