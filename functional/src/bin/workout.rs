use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
//  let expensive_result = simulated_expensive_calculation(intensity);
//  let expensive_closure = |num| {
//      println!("calculating slowly...");
//      thread::sleep(Duration::from_secs(2));
//      num
//  };
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
//          simulated_expensive_calculation(intensity)
//          expensive_result
//          expensive_closure(intensity)
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
//          simulated_expensive_calculation(intensity)
//          expensive_result
//          expensive_closure(intensity)
            expensive_result.value(intensity + 5)
        );
        println!(
            "Later, do {} pushups!",
//          simulated_expensive_calculation(intensity)
//          expensive_result
//          expensive_closure(intensity)
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
//              simulated_expensive_calculation(intensity)
//              expensive_result
//              expensive_closure(intensity)
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specific_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specific_value,
        simulated_random_number,
    );
}


struct Cacher<T>
where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            },
        }
    }
}
