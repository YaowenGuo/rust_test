mod ch;

fn main() {
    let first_name = "Pascal";
    let last_name = "Precht";
    let full_name = first_name.trim();
    greet(full_name);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}