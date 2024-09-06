use crate::lesson::Lesson;

pub struct Lesson7 { pub name: String }

#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f32,
}

impl Lesson for Lesson7 {
    fn run(&self) -> () {
        println!("\n{}:", self.name);

        let mut shuttle_1 = Shuttle {
            name: String::from("Test"),
            crew_size: 10,
            propellant: 1234.0,
        };

        let shuttle_2 = Shuttle {
            name: String::from("Test 2"),
            ..shuttle_1 // nice js syntax
        };

        shuttle_1.crew_size = 8;
        println!("Shuttle 1: {:?}", shuttle_1);
        println!("Shuttle 1: {:?}", shuttle_2);
    }
}