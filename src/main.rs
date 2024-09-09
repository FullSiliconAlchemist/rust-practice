mod lesson;
mod lesson_1;
mod lesson_2;
mod lesson_3;
mod lesson_4;
mod lesson_5;
mod lesson_6;
mod lesson_7;
mod lesson_8;
mod lesson_9;

use lesson::Lesson;
use lesson_2::Lesson2;
use lesson_1::Lesson1;
use lesson_3::Lesson3;
use lesson_4::Lesson4;
use lesson_5::Lesson5;
use lesson_6::Lesson6;
use lesson_7::Lesson7;
use lesson_8::Lesson8;
use lesson_9::Lesson9;

fn main() {
    let _t1 = Lesson1 { name: String::from("BASIC DATATYPES") };
    let _t2 = Lesson2 { name: String::from("FUNCTIONS") };
    let _t3 = Lesson3 { name: String::from("LOOPS AND CONTROL FLOW") };
    let _t4 = Lesson4 { name: String::from("OWNERSHIP") };
    let _t5 = Lesson5 { name: String::from("MODULES") };
    let _t6 = Lesson6 { name: String::from("INPUT AND OUTPUT") };
    let _t7 = Lesson7 { name: String::from("STRUCTS") };
    let _t8 = Lesson8 { name: String::from("GENERIC TYPES") };
    let _t9 = Lesson9 { name: String::from("TRAITS") };

    // _t1.run();
    // _t2.run();
    // _t3.run();
    // _t4.run();
    // _t5.run();
    // _t6.run();
    // _t7.run();
    // _t8.run();
    _t9.run();
}