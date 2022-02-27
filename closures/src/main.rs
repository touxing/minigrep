use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let intensity = 20;
    let random_number = 2;

    generate_workout(intensity, random_number)
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number > 3 {
            println!("take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, Option<u32>>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => v.unwrap(),
            None => {
                let v = (self.calculation)(arg.clone());
                self.value.insert(arg.clone(), Some(v));
                v
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }
}
