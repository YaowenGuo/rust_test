use std::cell::Cell;
use std::borrow::Borrow;
use std::rc::Rc;
use std::sync::Mutex;
use core::any::Any;

struct Parent {
    count: u32,
}

struct Child<'a> {
    parent: &'a Parent,
}

struct Combined<'a> {
    parent: Parent,
    child: Child<'a>,
}

impl<'a> Combined<'a> {
    fn new() -> Self {
        let parent = Parent { count: 42 };
        let child = Child { parent: &parent };

        Combined { parent, child }
    }
}

fn main() {


}

struct Node<'a, T: Any> {
    value: T,
    prev: &'a Option<Node<T>>,
    next: Option<Node<T>>,
}