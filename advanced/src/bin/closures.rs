fn add_one(x: i32) -> i32 {
    x + 1
}

/// fn is a concrete type and implements all of Fn, FnOnce and FnMut
/// So below we dont have to mark the param fn
/// By doing so users cannot pause closures to the function
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { 
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn do_twice_generic(f: impl Fn(i32) -> i32, arg: i32) -> i32 
{
    f(arg) + f(arg)
}

fn main() {
    let _answer = do_twice(add_one, 5);
    // let inc = 2;
    // let answer = do_twice(|x| x + inc, 5); // compilation error

    let _answer = do_twice_generic(|x| x + 1, 5);
    
    let _answer = returns_closure()(5);
}