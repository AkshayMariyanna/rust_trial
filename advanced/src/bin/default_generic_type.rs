//! Default Generic Type Parameters and Operator Overlaoding

//! Rust doesn’t allow you to create your own operators or overload arbitrary operators.
//! But you can overload the operations and corresponding traits listed in std::ops
//! by implementing the traits associated with the operator.

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point { // Default RHS is Self, so Point + Point
    type Output = Point; // Point + Point -> Point

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/*
definition of Add Trait

trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Output;
}
*/

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters { // RHS is Meters, so Millimeters + Meters
    type Output = Millimeters; // Millimeters + Meters -> Millimeters

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

fn main() {
    assert_eq!(Point {x: 1, y: 0} + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
    assert_eq!(Millimeters(500) + Meters(1), Millimeters(1500));
}
