use std::{fs, io};

use crate::lesson::Lesson;

pub struct Lesson11 { pub name: String }

impl Lesson for Lesson11 {
    fn run(&self) -> () {
        Lesson11::print_name(&self.name);

        let result = fs::read_to_string("the_ultimate_question.txt");
        let contents = match result {
            Ok(msg) => {
                println!("{}", msg);
                msg
            },
            Err(err) => {
                println!("Oh shiet: {}", err);
                let err_match = match err.kind() {
                    io::ErrorKind::NotFound => String::from("File not found!"),
                    io::ErrorKind::PermissionDenied => String::from("Permission denied"),
                    _ => String::from("Well you're boned!")
                };
                err_match
            }
        };
        println!("contents are: {:?}", contents);

        // CHALLENGE
        // Add error handling to lesson 5 - Modules
    }
}
