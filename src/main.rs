// Because lib and binary belong to different crate, so must use crate name to access the lib.
use ::rust_test::eat_at_restaurant;

fn main() {
    eat_at_restaurant()
}

