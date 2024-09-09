use crate::lesson::Lesson;

use std::fmt;

pub struct Lesson9 { pub name: String }

// Deriving a trait uses the default trait implementation
#[derive(PartialEq)]
struct Satellite {
    name: String,
    velocity: f64,
}

impl Satellite {
    fn new(name: &str, velocity: f64) -> Satellite {
        Satellite {
            name: String::from(name),
            velocity
        }
    }
}

trait Altitude<T> {
    fn calc(&self) -> ();
}

// Explicitly defined trait implementation for our comparison function
impl PartialOrd for Satellite {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.velocity.partial_cmp(&other.velocity)
    }
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n\tname: {}\n\tvelocity: {}", self.name, self.velocity)
    }
}

impl Altitude<Satellite> for Satellite {
    fn calc(&self) -> () {
        let g: f64 = 0.0000000000667430;
        let m: f64 = 5_972_000_000_000_000_000_000_000.0;
        let res = (g * m) / self.velocity * self.velocity;
        println!("The altitude is: {}", res);
    }
}

impl Lesson for Lesson9 {
    fn run(&self) -> () {
        Lesson9::print_name(&self.name);

        println!("\nCHALLENGE:");
        let hubble = Satellite::new("Hubble", 10.9);
        let voyager = Satellite::new("Voyager", 8.4);

        println!("The hubble stats: {}", hubble);

        hubble.calc();

        let res = hubble < voyager;
        println!("Comparing: {}", res);
    }
}
