/// Approx definition of vec!
#[macro_export]
macro_rules! vect {
    ($( $x:expr ), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let s = vect![1, 2, 3];

    println!("{:?}", s);
}
