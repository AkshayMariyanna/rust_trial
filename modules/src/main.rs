mod sound {
    fn guitar() {
    }

    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {}
        }
        pub fn clarinet() {}
    }

    mod voice {
    }
}


fn main() {
    crate::sound::instrument::clarinet();
    sound::instrument::clarinet();


    sound::instrument::woodwind::clarinet();
    {
        use crate::sound::instrument::woodwind;
        woodwind::clarinet();
    }
    //woodwind::clarinet(); //<- compilation error, as woodwind name not in scope here
    let mut v = plant::Vegetable::new("brinjal");

    v.name = String::from("eggplant");

    let _order1 = menu::Appetizer::Soup;
    let _order2 = menu::Appetizer::Salad;
}


mod plant {
    pub struct Vegetable {
        pub name: String, //public
        id: i32, // private
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}


mod menu {
    pub enum Appetizer {
        Soup, //public
        Salad,//public
    }
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}
fn function2() -> IoResult<()> {
    Ok(())
}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn pub_use_use() {
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
}

fn common_use() {
    {
        use std::cmp::Ordering;
        use std::io;
    }
    {
        use std::{cmp::Ordering, io};
    }

    {
        use std::io;
        use std::io::Write;
    }
    {
        use std::io::{self, Write};
    }

    {use std::collections::*;}
}
