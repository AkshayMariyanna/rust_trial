fn main() {
    let mut counter = 0;
    loop {
        counter+=1;
        println!("Again!");
        if counter == 10 {
            break;
        }
    }
    println!();
    loop_ret(2);
    println!();
    while_loop();
    println!();
    for_loop();
    println!();
    ranged_for();
}

fn loop_ret(mut x: i32) {
    let counter_res = loop {
        if x <= 0 {
            break 23
        }
        x -= 1;
        println!("again!");
    };

    println!("{}", counter_res);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("LIFTOFF!");
}

fn for_loop() {
    let arr = [1, 2, 3, 4, 5];

    for elem in arr.iter() {
        println!("{}", elem);
    }
}

fn ranged_for() {
    for elem in 3..0 {
        println!("{}", elem);
    }
    println!("LIFTOFF!");
}