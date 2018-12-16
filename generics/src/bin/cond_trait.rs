use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { // Self is syntactic sugar for the current type, particularly useful with generic implementations
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) { // self is syntactic sugar for "self: &Self", and also "&mut self" is short for "self: &mut Self"
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

trait TestTo_String {
    fn me_stringy(&self) -> String;
}

impl<T: Display> TestTo_String for T {
    fn me_stringy(&self) -> String { // Similar to how ToString trait is implemented in std
        String::from("I am available on all types that implement Display trait")
    }
}


fn main() {
    let x = String::from("Look at me");
    println!("{}", x);
    println!("{}", x.me_stringy());

    println!("Same with me - {}", 3);
    println!("{}", 3.me_stringy());
}
