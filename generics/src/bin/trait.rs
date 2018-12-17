use std::fmt::Debug;
use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
    fn default_summ(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn default_summ(&self) -> String { // override of a default fn, has no access to original fn
        String::from("That is all, move on")
    }
}
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(tweet);
}

fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
    println!("{}", item.default_summ());
}

fn notify_boundL<T: Summary> (item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_multi(_item1: impl Summary, _item2: impl Summary) {
}

fn notify_multi_bound<T: Summary> (_item1: T, _item2: T){
}

fn multiple_traits(_item: impl Summary + Display) {}

fn multiple_traits_bound<T: Summary + Display>(_item: T) {}

fn messy_multi<T: Display + Clone, U: Clone + Debug>(_t: T, _u: U) {}

fn where_clean_multi<T, U>(_t: T, _u: U)
where
    T: Display + Clone,
    U: Clone + Debug
{
}

fn trait_as_return() -> impl Summary { // appearance deceiving, can still return only one type, doesnt allow two types to be returned
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
    // if switch {
    //     NewsArticle {
    //         headline: String::from("Penguins win the Stanley Cup Championship!"),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from("The Pittsburgh Penguins once again are the best
    //         hockey team in the NHL."),
    //     }
    // } else {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("of course, as you probably already know, people"),
    //         reply: false,
    //         retweet: false,
    //     }
    // }
}
