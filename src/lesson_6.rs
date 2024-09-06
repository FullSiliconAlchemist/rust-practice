use std::{env, fs};
use std::path::Path;
// use std::io::Write;

use crate::lesson::Lesson;

pub struct Lesson6 { pub name: String }

impl Lesson for Lesson6 {
    fn run(&self) -> () {
        println!("\n{}:", self.name);

        let p = String::from("src/artifacts/planets.txt");
        check_path(&p);
        let contents = fs::read_to_string(p).unwrap_or_default();
        for line in contents.lines() {
            println!("{}", line);
        }

        let p = String::from("src/artifacts/planets.txt");
        let mut _planets_file = fs::OpenOptions::new().append(true).open(p).unwrap();
        // let _ = planets_file.write(b"\nPluto");

        challenge();
    }
}

fn challenge() -> () {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // For some reason the path check gets stored as env args
    if args.len() != 3 {
        println!("Missing args: Name and File.");
        return;
    }
    let file = &args[1];
    let name = &args[2];
    println!("{}, {}", name, file);
    check_path(&file);
    let contents = fs::read_to_string(file).unwrap_or_default();
    for line in contents.lines() {
        println!("{}", line);
        if line.contains(name) {
            println!("^ Found your guy!");
            return;
        }
    }
    println!("Guy not found...");
}

fn check_path(path: &str) -> () {
    let working_dir = std::env::current_dir().unwrap_or_default();
    let planets_path = Path::new(path);
    if !planets_path.exists() {
        println!("Path not found {}, working dir {}", planets_path.display(), working_dir.display());
        return;
    }
}