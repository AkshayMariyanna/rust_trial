fn main() {
    println!("Hello, a {:?} is {} cents!", Coin::Dime, Coin::Dime.value_in_cents());
    println!("Hello, a {:?} is {} cents!", Coin::Penny, Coin::Penny.value_in_cents());
    println!("Hello, a {:?} is {} cents!", Coin::Nickel, Coin::Nickel.value_in_cents());
    println!("Hello, a {:?} is {} cents!", Coin::Quarter(UsState::Alaska), Coin::Quarter(UsState::Alaska).value_in_cents());


    let some_u8_value = 0_u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8 = Some(1_u8);
    if let Some(3) = some_u8 {
        println!("three");
    } else {
        println!("Whew");
    }
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self{
            Coin::Penny => {
                println!("Lucky Penny");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            },
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
