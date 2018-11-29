fn main() {
    println!("Hello, world!");
    if_else(3);
    if_else(7);
    if_else_if();
    println!("{}", if_else_expr(4));
    println!("{}", if_else_expr(45));

    let number = 6;
    let _y = if number == 5 {
        50
    } else if number == 6 {
        100
    } else {
        95
    };
}

fn if_else(x: i32) {
    if x < 5 {
        println!("condition was true for {}", x);
    } else {
        println!("condition was false for {}", x);
    }
}

fn if_else_if() {
    let number = 6;
    if number % 4 == 0 {
        println!("{} divisible by 4", number);
    } else if number % 3 == 0 {
        println!("{} divisible by 3", number);
    } else {
        println!("{} not divisible by 4 or 3", number);
    }
}

fn if_else_expr(i: i32) -> i32 {
    if i < 5 {
        5
    } else {
        100
    }
}
