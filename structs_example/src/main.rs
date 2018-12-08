fn main() {
    println!("Area of a 50x30 rect params is {}", area(30, 50));
    println!("Area of a 50x30 rect tuples is {}", area_tup((30, 50)));
    let rect = Rectangle { width: 30, height: 50, };
    println!("Area of a 50x30 rect struct is {}", area_rect(&rect));
    println!("Area of a 50x30 rect struct with impl is {}", rect.area());
    println!("Printing rect debug: {:?}", rect);
    println!("Printing rect debug: {:#?}", rect);

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("A square of 20x20: {:?}", Rectangle::square(20));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size, }
    }
}

fn area_rect(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
