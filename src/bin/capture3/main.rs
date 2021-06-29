mod practice3_2;
mod practice3_6;
use std::mem;


fn main() {

    let story = String::from("Once upon a time...");

// Prevent automatically dropping the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.to_uppercase();
    println!("{}", capacity)
}