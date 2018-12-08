fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);


    let s = String::from("Hello, ") + "world"; // actual signature of add is (String, &str)
    println!("{}", s);

    let hello = String::from("Hello, ");
    let world = String::from("World");
    let s = hello + &world; // deref coercion -> in later lessons, &s2 = &s2[..] here
    println!("{}", s);


    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();

    let s = s1 + "-" + &s2 + "-" + &s3; // s1 gave up oenership to data after addition to s
    println!("1 - {}", s);

    let s1 = "tic".to_string(); // re-initializing tic as s1 gave up ownership
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("2 - {}", s);

    let s = "नमस्ते";
    for c in s.chars() {
        println!("{}", c);
    }
    for b in s.bytes() {
        println!("{}", b);
    }
}

fn hello() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
