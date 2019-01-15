use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

#[derive(HelloMacro)]
struct Dosa;

fn main() {
    Pancakes::hello_macro();
    Dosa::hello_macro();
}
