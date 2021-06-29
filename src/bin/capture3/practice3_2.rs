use std::io;
use std::io::Read;

pub fn read_word() {
    let mut str = String::new();
    io::stdin()
        .read_to_string(&mut str)
        .expect("Failed to read line");
    println!("{}", str);
}