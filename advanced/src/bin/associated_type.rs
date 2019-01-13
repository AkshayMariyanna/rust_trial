pub trait Iter1 {
    type Item; // Associated Type declaration

    fn next(&mut self) -> Option<Self::Item>; // -> Usage
    fn next_num(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            count: 0,
        }
    }
}

impl Iter1 for Counter {
    type Item = u32; // Associated type definition

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 4 {
            None
        } else {
            self.count += 1;
            Some(self.count)
        }
    }

    fn next_num(&mut self) -> Option<Self::Item> {
        if self.count > 4 {
            None
        } else {
            self.count += 1;
            Some(self.count)
        }
    }
}

// But why not use generics like below
pub trait Iter2<T> {
    fn next(&mut self) -> Option<T>;
}

impl Iter2<String> for Counter {
    fn next(&mut self) -> Option<String> {
        if self.count > 4 {
            None
        } else {
            self.count += 1;
            Some(format!("Item val: {}", self.count))
        }
    }
}

impl Iter2<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        if self.count > 4 {
            None
        } else {
            self.count += 1;
            Some(self.count)
        }
    }
}
// Using associated types instead of generics, forces only one implementation for a type

fn main() {
    let mut counter = Counter::new();
    while let Some(val) = counter.next_num() { // next_num has no conflicts -> no mentuion of trait or any genrics needed
        println!("{}", val);
    }

    let mut counter = Counter::new();
    while let Some(val) = Iter1::next(&mut counter) { // next has conflict with Iter2, so have to mention which next we want
        println!("{}", val);
    }

    let mut counter = Counter::new();
    while let Some(val) = Iter2::<String>::next(&mut counter) { // Genrics needed as Iter2 allows multiple impls [Also TURBOFISH]
        println!("{}", val);
    }

    let mut counter = Counter::new();
    while let Some(val) = Iter2::<u32>::next(&mut counter) { // Usage of generics to get a diff impl of Iter2
        println!("{}", val);
    }
}

/*
In other words, when a trait has a generic parameter,
it can be implemented for a type multiple times,
changing the concrete types of the generic type parameters each time.

When we use the next method on Counter,
we would have to provide type annotations to indicate which implementation of Iterator we want to use.
*/
