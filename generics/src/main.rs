// implementing generics in structs
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U>
where
    T: PartialOrd,
{
    fn greater(&self, other: &Point2<T, U>) -> bool {
        if self.x < other.x {
            return false;
        }
        return true;
    }
}

fn main() {
    let list = vec![1, 2, 4, 40, 10, 6, 5];
    let l = largest_num(&list);
    println!("The largest num is {l}");

    let list2 = vec!['y', 'm', 'a', 'q'];
    let lc = largest(&list2);
    println!("The largest char is {lc}");

    let p = Point { x: 5, y: 10 };
    let p1 = Point { x: 5.5, y: 6.6 }; // will work
                                       // let q = Point {x:5, y:10.0}; this will give error as we implemented only one generic
    println!("p:{:?}, p1: {:?}", p, p1);

    let p2 = Point2 { x: 5, y: 10.0 };
    let q2 = Point2 { x: 6, y: 20.0 };
    println!("{}", p2.greater(&q2));
}

fn largest_num(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// It support generic values
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
