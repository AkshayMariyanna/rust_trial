fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    unsafe {
        dangerous();
    }

    {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
    {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let (a, b) = split_at_mut(&mut v, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    undefined_behavior();
    c_abs();

    add_to_count(3);
}

unsafe fn dangerous() {}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

fn undefined_behavior() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    let _sl : &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn c_abs() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static _HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0; // Accessing or Modifying Static mutable variables is unsafe

fn add_to_count(inc: u32) {
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
    unsafe {
        COUNTER += inc;
    }
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Send and Sync traits are unsafe
unsafe trait Foo {
}

unsafe impl Foo for i32 {
}
