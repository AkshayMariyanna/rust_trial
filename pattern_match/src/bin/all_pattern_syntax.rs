fn main() {
    // Matching Literals
    {
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    // Matching Named Literals - scope doesnt let it
    {
        let x = Some(5);
        let y = 20;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {}", y), // this is not same as Some(20)
            _ => println!("Default case , x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {}", x, y);
    }

    // Multiple patterns
    {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    // Matching Ranges of Values with ...
    {
        let x = 'c';
        match x {
            'a' ... 'j' => println!("early ASCII letter"),
            'k' ... 'z' => println!("late ASCII letter"),
            _ => println!("Something else"),
        }
    }

    // Destructuring Structs
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        println!("a = {}, b = {}", a, b);

        let Point {x, y} = p;
        println!("x = {}, y = {}", x, y);

        // Now with literal values
        match p {
            Point{x: 0, y: 0} => println!("Origin"),
            Point{x, y: 0} => println!("On the x-axis at {}", x),
            Point{x: 0, y} => println!("On the y-axis at {}", y),
            Point{x, y} => println!("On neither axis: ({}, {})", x, y),
        }
    }

    // Destructuring Enums
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => println!("The Quit variant has no data to destructure."),
            Message::Move {x, y} => println!("Move in the x direction {} and in the y direction {}", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change the color to red {}, green {}, blue {}", r, g, b),
        }
    }

    // Destructuring nested structs and enums
    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Change the color to red {}, green {}, blue {}", r, g, b),
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Change the color to hue {}, saturation {}, value {}", h, s, v),
            _ => ()
        }
    }

    // Destructuring references
    {
        struct Point { x: i32, y: i32 }

        let points = vec![Point {x: 0, y: 0}, Point { x: 1, y: 5}, Point {x: 10, y: -3}];

        let sum_of_squares: i32 = points
            .iter()
            .map(|&Point{x, y}| x * x + y * y) // type of each item &Point
            .sum();
    }

    // Destructuring Sturcts and Tuples
    {
        struct Point {x: i32, y: i32}

        let ((feet, inches), Point{ x, y}) = ((3, 10), Point {x: 3, y: -10});
    }

    // Ignoring an Entire Value
    {
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }

        foo(3, 4);
    }

    // Ignoring parts of a value with a nested _
    {
        // The business requirements are that the user should not be allowed to overwrite an existing customization of a setting
        // but can unset the setting and can give the setting a value if it is currently unset.

        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);


        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            },
        }
    }

    // _ does not bind values but other matches do
    {
        let s = Some(String::from("Hello"));

        if let Some(_) = s { // _ does not bind value so s is not moved
            println!("Found a string");
        }
        println!("{:?}", s); // <- this is fine

        if let Some(_s) = s { // s is moved in this case, also _s just stops warnings, nothing special
            println!("Found a string");
        }

        // println!("{:?}", s); // <- this is not allowed
    }

    // Ignoring remaining parts of a value with ..
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point {x: 0, y: 0, z: 0};

        match origin {
            Point {x, ..} => println!("x is {}", x),
        }

        let numbers = (2, 4, 6, 8, 16, 32);

        match numbers {
            (first, .., last) => println!("Some numbers: {}, {}", first, last)
        }
    }

    // Extra Conditionals with Match Guards
    {
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }

        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if y == n => println!("Matched, n = {:?}", n),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);

        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }

        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    // @ Bindings
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id: id_variable @ 3...7 } => { // Here 'id_variable' is bound to match
                println!("Found an id in range: {}", id_variable)
            },
            Message::Hello { id: 10...12 } => { // Here 'id' is not bound to matched value
                println!("Found an id in another range")
            },
            Message::Hello { id } => { // Here 'id' is bound
                println!("Found some other id: {}", id)
            },
        }
    }

    {
        let robot_name = &Some(String::from("Bors"));

        match robot_name {
            Some(name) => println!("Found a name: {}", name),
            None => (),
        }

        println!("robot_name is: {:?}", robot_name);
    }

    {
        let robot_name = &Some(String::from("Bors"));

        match robot_name {
            &Some(ref name) => println!("Found a name: {}", name),
            None => (),
        }

        println!("robot_name is: {:?}", robot_name);
    }
}
