type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let _bs: Box<dyn std::fmt::Display> = Box::new(String::from("there"));
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    ()
}

fn returns_long_type() -> Thunk {
    Box::new(|| ())
}

fn x() -> ! { // the function x , returns never
    loop {
        println!("forecer");
    }
}

fn check_guess(guess: &str) -> u32 {
    match guess.trim().parse() {
        Ok(num) =>  num,
        Err(_) => panic!("Something wrong") // panic! has ! return so guess can be safely u32
    }
}

fn dont_call() -> ! {
    panic!("Why!?")
}

// Dynamically sized types adn Sized trait

fn dynamic_types() {
    // let string: str = "Hello "; // Size of these types cannot be fixed
    // let string: str = "Hello there"; 
    // let slice: [i32] = [1, 2];

    // All traits are dynamically sized types, so they similar to str and [T] can only exist as part of a pointer
    let s: &dyn std::fmt::Display = &String::from("Hello");
    // or 
    let bs: Box<dyn std::fmt::Display> = Box::new(String::from("there"));
}

fn generic_1<T>(t: T) {

}
// generic_1 and generic_2 are same, compiler adds Sized
fn generic_2<T: Sized>(t: T) {

}

// To use generics with dynamically sized types, you have to use T: ?Sized
// Special syntax only applicable to Sized trait and it reads - 
// "T may or may not be Sized"
fn generic_3<T: ?Sized>(t: &T) { // But then type of t, can no longer be T, here we used a reference

}