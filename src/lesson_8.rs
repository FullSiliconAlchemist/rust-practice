use crate::lesson::Lesson;

use::std::mem;
use std::ops::Add;

pub struct Lesson8 { pub name: String }

#[derive(Debug)]
struct Shape<T, U> {
    width: T,
    height: U,
}

impl<T, U> Shape<T, U> {
    pub fn new(width: T, height: U) -> Shape<T, U> {
        return Shape {
            width,
            height
        }
    }

    fn get_height(&self) -> &U {
        return &self.height;
    }
}

// Implementation only applicable to the proper type params
impl Shape<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        return 2 * self.height + 2 * self.width;
    }
}

impl Lesson for Lesson8 {
    fn run(&self) -> () {
        Lesson8::print_name(&self.name);

        let shape: Shape<&str, i32> = Shape::new("Explorer", 7);
        let proper_shape: Shape<u8, u8> = Shape::new(2, 5);

        println!("Shape height is {}", shape.get_height());
        println!("Proper shape perimeter is {}", proper_shape.get_perimeter());

        println!("Generic function");
        println!("Biggest is: {}", get_biggest("gooe", "helllo"));

        println!("BOXED values");
        // Considered "Smart pointers" moves any give value from the stack to the heap
        println!("Size (bytes) of a shape instance {}", mem::size_of_val(&shape));
        println!("Size (bytes) of a u8 shape instance {}", mem::size_of_val(&proper_shape));
        // Boxed version
        // Puts stack values into the heap
        let boxed_shape: Box<Shape<u8, u8>> = Box::new(proper_shape);
        // Size of pointer
        println!("Size (bytes) of a boxed u8 shape instance {}", mem::size_of_val(&boxed_shape));
        // Size of the Shape struct instance
        println!("Size (bytes) of a from box u8 shape instance {}", mem::size_of_val(&*boxed_shape));
        // Bring instance back on stack
        let deboxed_shape: Shape<u8, u8> = *boxed_shape;
        println!("Size (bytes) of a deboxed u8 shape instance {}", mem::size_of_val(&deboxed_shape));

        println!("\nCHALLENGE:");
        println!("Generic sum: {}", *sum_boxes(Box::new(1), Box::new(2)))
    }
}

fn sum_boxes<T>(a: Box<T>, b: Box<T>) -> Box<T>
where T: Add<Output = T> + Copy {
    let res = *a + *b;
    return Box::new(res);
}

fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    } else {
        return b;
    }
}