#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(::add_2(2), 4); // PartialEq and Debug traits necessary
    }

    #[test]
    fn explorationi_1() {
        assert_eq!(::add_3(2), 5, "Expected 5 but got {}", ::add_3(2));
    }

    #[test]
    fn another() {
        panic!("This test fails");
    }

    #[test]
    fn it_works() -> Result<(), String> { // Result<T, E> can also be used instead of panic
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn ignored() {
        assert!(2 == 3);
    }
}

pub fn add_2(a: i32) -> i32 {
    a + 2
}

fn add_3(a: i32) -> i32 {
    a + 2
}
