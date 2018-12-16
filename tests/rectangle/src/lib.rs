#[cfg(test)]
mod tests {
    use rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = rectangle::Rectangle { length: 8, width: 7 };
        let smaller = rectangle::Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }
}

mod rectangle {
    #[derive(Debug)]
    pub struct Rectangle {
        pub length: u32,
        pub width: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }
}
