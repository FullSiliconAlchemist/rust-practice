use crate::lesson::Lesson;

pub struct Lesson2 { pub name: String }

impl Lesson for Lesson2 {
    fn run(&self) -> () {
        println!("\n{}:", self.name);

        say_hello();
        say_a_number(21);

        let (init, res) = square(32);
        // last arg is debug print
        println!("the square of {} is {:?}", init, res);
        // Challenge:
        let (c, f) = celsius_to_farenheit(23.0);
        println!("temps {}C is {}F", c, f);
    }
}

fn celsius_to_farenheit(celsius: f32) -> (f32, f32) {
    return (celsius, (1.8 * celsius) + 32.0);
}

// Represents void
fn say_hello() -> () {
    println!("Hello World!")
}

fn say_a_number(num: i32) {
    println!("the number is {}", num);
}

fn square(num: i32) -> (i32, i32) {
    return (num, num * num);
}