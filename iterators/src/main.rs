pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    // 1. v1.iter() Imuuatable borrow
    // 2. v1.iter_mut() Mutable borrow
    // 3. v1.into_iter() Move ownership

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let total: i32 = v1.iter().sum();
    assert_eq!(total, 6);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    let v3:Vec<_> = v2.into_iter().filter(|x| *x==3).collect();
    assert_eq!(v3, vec![3]);
}
