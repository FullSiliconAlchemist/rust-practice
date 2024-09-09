use crate::lesson::Lesson;

pub struct Lesson7 { pub name: String }

#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f32,
}

impl Shuttle {
    // Struct constructor methodology
    // Also known as an associated function (does not mutate instance)
    pub fn new(name: &str, crew_size: u8) -> Shuttle {
        return Shuttle {
            name: String::from(name),
            crew_size,
            propellant: 0.0
        }
    }

    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn add_fuel(&mut self, gallons: f32) -> () {
        self.propellant += gallons;
    }

    fn set_crew_size(&mut self, size: u8) -> () {
        self.crew_size = size;
    }
}

struct RGB (u8, u8, u8);

impl Lesson for Lesson7 {
    fn run(&self) -> () {
        Lesson7::print_name(&self.name);

        // No constructor
        let shuttle_1 = Shuttle {
            name: String::from("Test"),
            crew_size: 10,
            propellant: 1234.0,
        };

        let shuttle_2 = Shuttle {
            name: String::from("Test 2"),
            ..shuttle_1 // nice js syntax
        };

        // With constructor
        let mut shuttle_3 = Shuttle::new("Explorer", 7);

        shuttle_3.set_crew_size(8);
        shuttle_3.add_fuel(2000.0);

        println!("Shuttle 1: {:?}", shuttle_1);
        println!("Shuttle 2: {:?}", shuttle_2);
        println!("Shuttle 3: {:?}", shuttle_3);

        // Tupples
        let red = RGB(255, 0, 0);
        println!("R value is: {}", red.0);
    }
}