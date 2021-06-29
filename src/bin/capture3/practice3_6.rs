use std::ops::Index;

#[test]
fn replace_char() {
    let mut str = "Hello";
    unsafe {
        // for nth in str.as_bytes_mut() {
        //     nth = b'X';
        // }
        // let mut bytes = str.as_mut_ptr().into_bytes();
        // bytes[0] = b'X';
        //
        // let bytes = s.as_bytes();
    }
    // str[0] = 'X';
    // println!("{}", str);

    // let index : usize = 0;
    // str[index] = 'X';
    //
    // let mut s = "poyo".to_string();
    // let mut bytes = s.into_bytes();
    // bytes[1] = 'i' as u8;
    //
    // unsafe { s = String::from_utf8_unchecked(bytes) }

    let s: String = "Hello, World!".to_string();
    let ss: String = s.chars()
        .map(|x| match x {
            '!' => '?',
            'A'..='Z' => 'X',
            'a'..='z' => 'x',
            _ => x,
        })
        .collect();
    println!("{}", s);

    // println!("{}", str);
}