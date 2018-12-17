fn main() {
    let literal_string = "I am literal string, I am immutable";
    println!("{}", literal_string);
    let mut mutable_string = String::from("String::from creates a ");
    mutable_string.push_str("mutable string");
    println!("{}", mutable_string);

    let _s1 = gives_ownership();

    let s2 = String::from("hello");

    let _s3 = takes_and_gives_back(s2);
}

fn move_lesson() {
    let x = 5;
    let _y = x;

    let s1 = String::from("String for move"); 
    let _s2 = s1; // move

    let s1 = String::from("String for clone");
    let _s2 = s1.clone();

    // Copy and Drop Traits -> in future episodes
}


fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
