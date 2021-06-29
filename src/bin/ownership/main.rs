mod circle;

use std::marker::Copy;
#[derive(Debug, Copy, Clone)]

struct Point<T, U>{
    x: T,
    y: U,
}

struct IntPoint {
    x: i32,
    y: i32,
}

fn main() {
    // --------------
    // let x = 5;
    // let mut y = x;
    // y = 6;
    // println!("x: {}, y: {}", x, y)

    // --------------
    // s1 被移动给 s2 后，即使 s2 有了新值，s1 没有新赋值的情况下依旧不再可用
    // let s1 = String::from("hello");
    // let mut s2 = s1;
    // s2 = String::from("world!");
    // println!("x: {}, y: {}", s1, s2)


    // --------------
    // 结构体虽然大小固定，但是依然不是复制的。
    // let p1 = IntPoint { x: 1, y: 2 };
    // let p2 = p1;
    //
    // println!("p1: {:?}, p1: {:?}", p1, p2)

    // 实现了 Copy trait 的结构器，可以是 Copy 的。
    // #[derive(Debug, Copy, Clone)] // 实现方式 1. 实现 Copy 也必须实现 Clone.
    // let p1 = Point { x: 1, y: 2 };
    // let p2 = p1;
    //
    // println!("p1: {:?}, p1: {:?}", p1, p2)


    // --------------
    // let s = String::from("hello");  // s comes into scope
    //
    // takes_ownership(s);             // s's value moves into the function...
    // // ... and so is no longer valid here
    // print!("s: {}", s)


    // ----------------
    // let s1 = gives_ownership();         // gives_ownership moves its return
    // // value into s1
    // print!("s: {}", s1);
    //
    // let s2 = String::from("hello");     // s2 comes into scope
    //
    // let s3 = takes_and_gives_back(s2);  // s2 is moved into
    // // takes_and_gives_back, which also
    // // moves its return value into s3


    // ----------------
    // let s1 = String::from("hello");
    //
    // let len = calculate_length(&s1);
    //
    // println!("The length of '{}' is {}.", s1, len);


    // ------------------ reference ---------------

    // let s1 = String::from("hello");
    //
    // let s: &String;
    // s = &s1;
    //
    // println!("The reference of s1 {} is {}.", s1, *s);


    // ----------------- lifetime of reference -------

    // let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // // <---- r1 and r2 are no longer used after this point
    //
    // let r3 = &mut s; // no problem
    // println!("{}", r3);
    // println!("r1: {}, r3: {}",r1, r3); // BIG PROBLEM
    test_scale();
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

// reference
fn calculate_length(s: &String) -> usize {
    s.len()
}


fn test_scale() {
    let mut s = String::from("hello");

    let r1 = & s; // 可变引用。
    let r2 = &mut s; // 不合法
    println!("{}, {}", r1, r2);
}