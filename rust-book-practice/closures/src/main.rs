use std::thread;
use std::time::Duration;
use std::collections::HashMap;

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
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut expensive_reslut = Cacher::new(expensive_closure);

    println!("Generating...");
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_reslut.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_reslut.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_reslut.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_user_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_user_random_number
    );
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
