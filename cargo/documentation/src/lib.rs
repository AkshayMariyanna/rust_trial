//! # Documentation
//!
//! `documentation` is a crate demonstrating the kinds of documentation available with cargo.
//!
//! One can run `cargo doc --open` to try this crate
//!
//! It also has a function that adds *one*

/// Adds one to the number given
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, documentation::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(3), 4);
    }
}
