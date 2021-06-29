use std::io;

fn main() {
    // println!("Guess the number!");
    //
    // println!("Please input your guess:");
    //
    // let mut guess = String::new();
    //
    // let ioOutput = io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line!");
    //
    // println!("You guessed: {}", guess);
    //
    // println!("Return value is: {}", ioOutput);
    let mut i = 1;
    // let mut sum = 0;
    // while i <= 10 {
    //     sum += i;
    //     i += 1;
    // }
    println!("i value is: {}", i);
    let i = "hello";
    println!("Sum of 1 to 10 is: {}", i);

    // let a: i32;
    //
    // {
    //     // print!("value of a: {}", a); // 无法通过，不能有空值
    //     a = 123;
    //     print!("value of a: {}", a);
    // }

    scope();
}

fn tow_arguments_function(x: i32, y: i32) {
    println!("The argument is x: {}, y: {}", x, y);
}


fn control_flow_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn control_flow_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn control_flow_match() {
    let mut number = 3;

    match number {
        3 => println!("3"),
        _ => println!(" else")
    }

    println!("LIFTOFF!!!");
}


fn scope() {
    {
        let a = 3;
        let b = a;
        println!("Is a available: {}, {}", a, b);

        let s = String::from("hello world!");
        let s1 = s;
        println!("{}, {}", s, s1); // Will not working.
    }
}