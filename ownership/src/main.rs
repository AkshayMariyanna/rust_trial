fn main() {
    let literal_string = "I am literal string, I am immutable";
    println!("{}", literal_string);
    let mut mutable_string = String::from("String::from creates a ");
    mutable_string.push_str("mutable string");
    println!("{}", mutable_string);
}
