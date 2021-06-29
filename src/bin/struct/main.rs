mod unit_like_struct;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u64 {
        (self.width as u64) * (self.height as u64)
    }

    fn height(&mut self, size: u32) -> &Rectangle {
        self.height = size;
        return self;
    }
}

impl Rectangle {
    fn can_hole(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 32,
        height: 3
    };
    let rect2 = Rectangle {
        width: 4,
        height: 3,
    };

    println!("The rect1 rectangle is: \n{:#?}", rect1);
    println!("The area of rect1 is: {}", rect1.area());
    println!("The rect2 rectangle is: \n{:#?}", rect2);
    println!("The area of rect2 is: {}", rect2.area());

    println!("The rect1 can hold rect2: {}", rect1.can_hole(&rect2));

    rect1.height(4);

}