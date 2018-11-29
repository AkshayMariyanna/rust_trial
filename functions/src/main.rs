fn main() {
    println!("Hello, world!");
    
    another_func();
    param_func(5);
    scopes_are_expressions();
    let _five = five();

    println!("Adding 1 to 5 is {}", add_one(_five));

    let _empty_tuple = ();
}

fn another_func() -> () {
    println!("Another function");
}

fn param_func(i: i32) {
    println!("The integer parameter is {}", i);
}

fn scopes_are_expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("Value from scope expression is {}", y);
}

fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}
