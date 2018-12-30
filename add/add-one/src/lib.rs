use rand;
use rand::*;

/// Adds one
///
/// # Example
/// ```
/// let a = 2;
///
/// assert_eq!(3, add_one::add_one(a));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn ret_rand(mut random_g: rand::ThreadRng) -> u32 {
    random_g.next_u32()
}

#[cfg(test)]
mod tests {
    use super::add_one;
    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
