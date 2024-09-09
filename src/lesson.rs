pub trait Lesson {
    fn run(&self) -> ();

    fn print_name(name: &str) -> () {
        println!("\n{}", name);
    }
}