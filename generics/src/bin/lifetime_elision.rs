/*
 * one of the ELISIONS rust compiler has
 * without this inference, declaration would be

fn first_word<'a>(s: &'a str) -> &'a str

*/
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str { // self ellision
        println!("Attention please: {}", announcement);
        self.part
    }
}

// All string literals have static lifetime, meaning the reference is active throughought the programs life
// 'static

// let s: &'static str = "I have a static lifetime";
