mod pair;
mod largest;


struct Point<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T, T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


pub use largest::largest;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    //
    // let p1 = Point { x: 5, y: 10.4 };
    // let p2 = Point { x: "Hello", y: 'c' };
    //
    // let p3 = p1.mixup(p2);
    //
    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    //
    // // new
    //
    // let pair = Pair::new(1, 2);
    // println!("The largest char is {}", result);
}