mod counter;

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoes_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoes_size).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") },
            ]
        );
    }
}

fn main() {
    let v1:Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    for val in &v1 {
        println!("item: {}", val);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let fruits=["mango", "apple", "banana", "litchi", "watermelon"];
    for a in &fruits
    {
        println!("{} ", a);
    }

    let v = (1, 2, 3, 4, 5, 6);
    //s的类型是  Filter<Map<Iter<i32>, ...>, ...>
    // println!("{:?}", (1..10).sum::<i32>());
    // let s = v.map(|x| {x + 1}).filter(|x| x % 5 != 0);
}