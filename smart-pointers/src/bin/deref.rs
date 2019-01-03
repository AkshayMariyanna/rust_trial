use std::ops::{Deref, DerefMut};

fn main() {
    let x = 5;
    let y = &x;

    let b = Box::new(x);
    let myb = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, y); // this line won't compile
    assert_eq!(5, *y);

    assert_eq!(5, *b);
    assert_eq!(5, *myb);

    let m = MyBox::new(String::from("Rust"));

    // Deref Coercion
    // &m -> (m.deref()) // m.deref returns &String
    // &String -> (String.deref()) which is of type &str
    hello(&m);

    let mut act = MyBox(String::from("Rust"));
    let act3 = &mut act;
    let act1: &mut String = act3;
    // let act2: &String = act3;
    println!("{}", act1);
    act1.push_str("y nail");
    println!("{}", *act)
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {

    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
