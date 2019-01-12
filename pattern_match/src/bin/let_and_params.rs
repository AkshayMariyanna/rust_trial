fn main() {
    let p = (PD(1), 2, 3);
    // let (_x, _, _) = p;
    // let (_x, ..) = p;
    print_cordinates_pdv(&p);
    let asp = (1, 2, 3);
    print_cordinates(&asp);
    print_cordinates_1(&p);
}


#[derive(Debug)] // , Clone, Copy)] // if Copy trait was implemented then &(x, y, z) is possible
struct PD(i32);

fn print_cordinates_pdv((x, y, z): &(PD, i32, i32)) { // no &(x, y, z), otherwise compiler error
    let refpd: &PD = x; // x is a reference

    let &PD(x1) = refpd; // pattern match considers & as well, to deduce whether an object is required or a reference to one
    let PD(x2) = refpd;

    let _x1: i32 = x1;
    let x2: &i32 = x2;
    println!("Current location through refs: ({:?}, {}, {})", x2, y, z);
}

fn print_cordinates(&(x, y, z): &(i32, i32, i32)) { // notice &(x, y, z) -> that matches and copies
    // let refx : &i32 = x; // Type mismatch cause x is not a reference
    println!("Current location through copies: ({:?}, {}, {})", x, y, z);
}

fn print_cordinates_1(&(PD(x), y, z): &(PD, i32, i32)) {
    let _x1: i32 = x;
    println!("Current location through copies but from PD: ({:?}, {}, {})", x, y, z);
}
