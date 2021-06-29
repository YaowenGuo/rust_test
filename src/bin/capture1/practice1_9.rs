use std::io;


#[test]
fn test_sum() {
    let mut sum = 0;
    for i in 50..101 {
        sum += 1;
    }
    println!("Sum of 50 to 100 is: {}", sum);
}

#[test]
fn test_dec() {
    for i in (1..10).rev() {
        println!("{}", i);
    }
}

//[test]
// 直接运行会阻塞，没搞清楚 test 如何往标准输入里增加参数。可以直接在 main 函数中调用。
pub fn test_input() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut s = input.split_whitespace();
    let mut a: i32 = s.next().unwrap().parse().unwrap();
    let mut b: i32 = s.next().unwrap().parse().unwrap();

    if a > b {
        let tmp = a;
        a = b;
        b = tmp;
    }
    for i in (a + 1)..b {
        println!("{}", i);
    }
}