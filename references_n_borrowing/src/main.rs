fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // s1 is borrowed

    println!("The length of '{}' is {}", s1, len);

    let mut s2 = s1; //s1 is moved, use clone if want to copy
    change(&mut s2);

    let len = calculate_length(&s2);
    println!("The length of '{}' is {}", s2, len);

    {
        let _s5 = &mut s2; // This is okay
    }
    let _s3 = &mut s2;
    // let _s4 = &mut s2; Only one mutable reference of a val in a scope

    let mut s = String::from("hello");
    {
        let _r1 = &s;
        let _r2 = &s;
        // let _r3 = &mut s; Cannot have a mutable reference when an immutable ref is in scope
    }
    let _r4 = &mut s;
    _r4.push_str(", world");

    let _s = String::from("hello world");
    let s1 = String::from("hello world");
    println!("{}", _r4);

    let _hello: &str = &s1[0..5];
    let _world = &s1[6..11];

    let _hello = &s1[0..=4];
    let _world = &s1[6..=10];
    // s1.clear(); Error
    // s1.push_str("some more"); Erro


    let string_literal: &str = "Hello, I am a slice too";
    let _wd = accept_literals_and_strings(string_literal);
    let _wd = accept_literals_and_strings(&s1[..]);
    let _wd = accept_literals_and_strings(&string_literal[..]);
}

fn accept_literals_and_strings(s: &str) -> &str {
    &s[..]
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn ref_pass(s: &String) -> &String {
    s
}

fn own_pass() -> String {
    let s = String::from("hello");

    s
}

// Below wont compile
// fn dangle() -> &String {
//     let s = String::from("hello");
// 
//     &s
// }
