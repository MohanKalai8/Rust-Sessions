#[derive(Debug)]
struct Rectange {
    width : u32,
    height : u32,
}

impl Rectange {
    fn new(width: u32, height:u32) -> Rectange {
        Rectange {
            width,
            height
        }
    }

    fn area(&self) -> u32{
        self.width * self.height 
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self,other:&Rectange) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rectangle = Rectange::new(10,5);
    println!("The rectangle is {:?}",rectangle);
    println!("The rectangle is {:#?}", rectangle);
    dbg!(&rectangle);
    
    let area = rectangle.area();
    println!("The area of the rectangle is {area}");

    let perimeter = rectangle.perimeter();
    println!("The perimeter of the rectanlge is {perimeter}");

    let another = Rectange::new(5,2);
    let can_hold = rectangle.can_hold(&another);
    println!("The another rectangle fit in main rectangle : {can_hold}");
}
