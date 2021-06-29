use std::cmp::PartialOrd;
use std::marker::Copy;

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


pub fn largest_i32(list: &[i32]) {

}