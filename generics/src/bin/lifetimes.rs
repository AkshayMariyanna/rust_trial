fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    acceptable();
}

fn acceptable() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn not_acceptable1() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    println!("The longest string is {}", result);
}

fn not_acceptable2() {
    let mut string1 = String::from("long string is long");

    {
        let mut string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        let imakecopy = &mut string1[..];
        let imakecopy2 = &mut string2[..];
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn ret_scope_unacceptable<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("some string");
    result.as_str()
}
