fn main() {
    let _guess: u32 = "42".parse().expect("Not a number");

    let _ami64 = 42_i64;
    let _amf32: f32 = 4.3;
    let _ambool = false;
    tuples();
    arrays();
}

fn tuples() {
    let tup = (1, 2.3, "saa");

    let (_, y, _) = tup;

    println!("The middle value of tuple is {}", y);
    println!("The first and third value of tuple are {}, {}", tup.0, tup.2);
}

fn arrays() {
    let arr: [f64; 3] = [2.3, 4.67, 9.999];
    println!("1st: {}, 2nd: {}, 3rd: {}", arr[0], arr[1], arr[2]);

    println!("Now I crash accessing invalid index on an array");
    let index = 4;
    let _s = arr[index];
}