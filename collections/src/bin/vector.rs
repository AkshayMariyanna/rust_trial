fn main() {
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);

    let v = vec![1, 2, 3, 4, 5];
    let _third: &i32 = &v[2];
    let _third: i32 = v[2]; // ints are copied by default, so this is fine

    let v = vec![T {i: 2,}];
    let _first = &v[0];
    //let first = v[0]; // this is not fine, cause move mutates the original vector

    let _v = vec![T {i: 2}]; 
    // let first = v[0]; // this also is not fine, as a read on vector mutates vector

    let v = vec![1,2,3,4,5];
    let v_index = 5;
    // let sixth = &v[v_index]; // this causes a panic, as index out of bounds
    // println!("Fourth i {}", sixth);

    match v.get(v_index) {
        Some(val) => println!("Reacheable element {} at index {}", val, v_index),
        None => println!("Unreachable element at index {}", v_index),
    };

    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}",i);
    }

    let mut v = vec![5, 6, 7];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}",i);
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.4),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

struct T {
    i: i32,
}
