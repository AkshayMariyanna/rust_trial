fn main() {
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    const MAX_POINTS: u32 = 100000;
    println!("The value of c is : {}", MAX_POINTS);

    let y = 10;
    let y = y*2;
    let y = y/3;

    println!("The value of y is : {}", y);

    let space = "   ";
    let space = space.len(); // fine, cause immutable vars can change type
    println!("The number of spaces is {}", space);

    // let mut space1 = "   ";
    // space1 = space1.len(); // not fine, mutable vars cannot change type 
}
