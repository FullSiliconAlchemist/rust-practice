use std::collections::HashMap;

use crate::lesson::Lesson;

pub struct Lesson12 { pub name: String }

impl Lesson for Lesson12 {
    fn run(&self) -> () {
        Lesson12::print_name(&self.name);

        // Vectors
        let mut astronauts: Vec<String> = Vec::new();
        astronauts.push(String::from("Buzz"));
        println!("Astronauts: {:?}", astronauts);

        let a = astronauts.pop();
        println!("{:?}", a);

        let b = astronauts.get(0);
        println!("{:?}", b);

        // Best example of defining macros right here - shorthand syntax
        let countdown = vec![5, 4, 3, 2, 1, 0];
        println!("{:?}", countdown);

        // Hashmaps
        let mut missions_flown = HashMap::new();
        missions_flown.insert(String::from("Buzz"), 5);
        missions_flown.insert(String::from("Hadfield"), 3);
        println!("Missions flown: {:?}", missions_flown);

        let result = missions_flown.get("Buzz");
        println!("Buzz? {:?}", result);

        missions_flown.entry(String::from("Hadfield")).or_insert(6);

        let result = missions_flown.get("Buzz");
        println!("Hadfield? {:?}", result);

        // In other words: Add if not exist
        missions_flown.entry(String::from("Bob")).or_insert(6);
        println!("Missions flown: {:?}", missions_flown);
    }
}
