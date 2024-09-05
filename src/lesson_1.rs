use crate::lesson::Lesson;

pub struct Lesson1 { pub name: String }

impl Lesson for Lesson1 {
    fn run(&self) -> () {
        println!("\n{}:", self.name);

        let mut x = "hello";
        println!("hello world");
        println!("this is: {}", x);
        x = "bob";
        println!("My name is: {}", x);
        let mut y: u16 = 250;
        y = y + 1000;
        println!("My name is: {0} and I am {1} years old.", x, y);
        // bitwise
        let b = 0b1111_1011u8;
        println!("This number is: {}", b);
        println!("This bit is: {:08b}", b);
        // Inversion
        println!("This bit is: {:08b}", !b);
        // And
        println!("The value of the last bit is: {:08b}", b & 0b0000_0001);
        // Or
        println!("This bit is: {:08b}", b | 0);
        // XOr
        println!("This bit is: {:08b}", b ^ 0b0000_1101);
        // Shift
        println!("This bit is: {:08b}", b << 4);
        println!("This bit is: {:08b}", b >> 1);
        // Short circuiting
        let p = true || panic!(); // Does not throw error cause true and OR always results in true
        println!("{}", p);
    
    }
}