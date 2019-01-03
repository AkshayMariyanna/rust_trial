use std::rc::Rc; // Rc<T> in itself only gives read access to data
// Interior Mutability Pattern is available if both mutable reference and multiple ownership are required(RefCell<T>)

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use self::List::{ Nil, Cons };

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
