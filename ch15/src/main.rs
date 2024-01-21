use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::sync::Mutex;

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    // int* p = new int(5);
    // delete p;
    // int array[5];
    // int* p1 = array;
    println!("b = {b}");

    let l = List::from_array(&vec![1, 2, 3]);
    println!("l = {l}");

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a = MyBox(x);
    let s = MyBox(String::from("Hello"));

    need_str(&s);
    need_str(&(*s)[..]);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *a);
    assert_eq!(5, *a.deref());

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    // [5]
    let a = Rc::new(Cons(5, RefCell::new(Tail::Rc(Rc::new(Nil)))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // [10, a]
    let b = Rc::new(Cons(10, RefCell::new(Tail::Weak(Rc::downgrade(&a)))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // a = [5, b] -> a = [5, [10, a]]
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Tail::Weak(Rc::downgrade(&b));
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {}", a.tail().unwrap().borrow());
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping .... `{}`", self.data);
    }
}

fn need_str(s: &str) {}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
enum Tail {
    Rc(Rc<List>),
    Weak(Weak<List>),
}

impl Tail {
    fn is_empty(&self) -> bool {
        match self {
            Tail::Rc(r) => r.is_empty(),
            Tail::Weak(w) => w.upgrade().map_or(false, |r| r.is_empty()),
        }
    }
}

// 1 -> 2 -> 3 -> Nil
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Tail>),
    Nil,
}

impl List {
    fn head(&self) -> Option<i32> {
        match self {
            Cons(head, _) => Some(*head),
            Nil => None,
        }
    }

    fn tail(&self) -> Option<&RefCell<Tail>> {
        match self {
            Cons(_, tail) => Some(tail),
            Nil => None,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Cons(_, _) => false,
            Nil => true,
        }
    }

    fn from_array(array: &[i32]) -> List {
        if let [head, tail @ ..] = array {
            Cons(
                *head,
                RefCell::new(Tail::Rc(Rc::new(List::from_array(tail)))),
            )
        } else {
            Nil
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cons(head, tail) => write!(
                f,
                "{head}{}",
                if tail.borrow().is_empty() { "" } else { ", " }
            )
            .and(Display::fmt(&tail.borrow(), f)),
            Nil => Ok(()),
        }
    }
}

impl Display for Tail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tail::Rc(r) => Display::fmt(&r, f),
            Tail::Weak(w) => w
                .upgrade()
                .ok_or(std::fmt::Error)
                .and_then(|r| Display::fmt(&r, f)),
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        println!("drop list: {}", self.head().unwrap_or_default());
    }
}
