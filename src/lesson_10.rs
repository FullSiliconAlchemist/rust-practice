use crate::lesson::Lesson;

use std::fmt;

pub struct Lesson10 { pub name: String }

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle,
    Triangle
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle => 0.0,
            Shape::Triangle => 69.696969
        }
    }
}

#[derive(Debug)]
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Unknown => write!(f, "Location is : {:?}", self),
            Location::Anonymous => write!(f, "Location is : {:?}", self),
            Location::Known(a, b) => write!(f, "Exact coordinates are: {}, {}", a, b),
        }
    }
}

impl Location {
    fn get_location(&self) -> () {
        println!("My location is {:?}", self);
    }
}

impl Lesson for Lesson10 {
    fn run(&self) -> () {
        Lesson10::print_name(&self.name);

        // Example 1
        let my_shape = Shape::Rectangle;
        println!("My shape is: {:?}", my_shape);
        let circ = Shape::Circle(2.0);
        println!("My shape is: {:?}", circ);

        match my_shape {
            Shape::Circle(r) => println!("This circle has a radius of {}", r),
            Shape::Rectangle => println!("This is a rectangle"),
            Shape::Triangle => {
                println!("This is a triangle in a code block")
            }
        }

        // Exmple 2
        let number = 1u8;
        let result = match number {
            0 => "zero",
            1 => "one",
            _ => "something else" // wildcard
        };
        println!("Result -> {}", result);

        // Example 3
        let p = circ.get_perimeter().trunc();
        println!("Shape perimeter is {}", p);

        // Example 4
        let arr = [1, 2, 3, 4, 5];
        let num = arr.get(5);
        let res = *num.unwrap_or(&-1);
        println!("number is: {:?}", res);

        let res_alt = match num {
            Some(num) => num,
            None => &-1
        };
        println!("number is: {}", res_alt);

        // Example 5
        let a = Some(13);
        // Only executes if a is of that type
        if let Some(13) = a {
            println!("Thirteen");
        }

        // CHALLENGE
        let loc = Location::Known(1000.0, 2000.0);
        loc.get_location();
        println!("Display loc: {}", loc);
    }
}
