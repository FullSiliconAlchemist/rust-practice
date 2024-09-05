mod lesson;
mod lesson_1;
mod lesson_2;
mod lesson_3;
mod lesson_4;
mod lesson_5;

use lesson::Lesson;
use lesson_2::Lesson2;
use lesson_1::Lesson1;
use lesson_3::Lesson3;
use lesson_4::Lesson4;
use lesson_5::Lesson5;

fn main() {
    let _t1 = Lesson1 { name: String::from("BASIC DATATYPES") };
    let _t2 = Lesson2 { name: String::from("FUNCTIONS") };
    let _t3 = Lesson3 { name: String::from("LOOPS AND CONTROL FLOW") };
    let _t4 = Lesson4 { name: String::from("OWNERSHIP") };
    let _t5 = Lesson5 { name: String::from("MODULES") };

    // _t1.run();
    // _t2.run();
    // _t3.run();
    // _t4.run();
    _t5.run();
}