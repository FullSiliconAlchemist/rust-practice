use crate::lesson::Lesson;

pub struct Lesson3 { pub name: String }

impl Lesson for Lesson3 {
    fn run(&self) -> () {
        println!("\n{}:", self.name);

        // Shorthand syntax for conditional assignment
        let x = if true {2} else {3};
        println!("{}", x);

        // Loop (kinda like while)
        let loop_result = loop_till(1);
        println!("Loop res {}", loop_result);

        // For loop
        let new_string = loop_through_array(&['h', 'e', 'l', 'l', 'o']);
        println!("The string is {}", new_string);
        multi_dim_mut_array(&mut[&mut[1, 2, 3, 4], &mut[5, 6, 7, 8]]);

        // Challenge
        let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];

        /* YOUR CODE GOES HERE */
        let (min, max, mean) = min_max_mean(&numbers);

        assert_eq!(max, 56);
        assert_eq!(min, -18);
        assert_eq!(mean, 12.5);
        println!("Tests passed!");
    }
}

fn min_max_mean(arr: &[i32]) -> (i32, i32, f64) {
    assert!(arr.len() > 0);

    let mut min: i32 = arr[0];
    let mut max: i32 = arr[0];

    let mut acc: f64 = 0.0;

    for val in arr.iter() {
        if val < &min {
            min = *val;
        } else if val > &max {
            max = *val;
        }

        acc += *val as f64;
    }

    return (min, max, acc / arr.len() as f64);
}

fn multi_dim_mut_array(arr: &mut [&mut [i32]]) -> () {
    for i in arr.iter_mut() {
        for j in i.iter_mut() {
            *j *= *j;
            print!("{}\t", j);
        }
        println!();
    }
}

fn loop_through_array(arr: &[char]) -> String {
    let mut result = String::new();
    for (index, &char) in arr.iter().enumerate() {
        println!("Char {} at index {}", char, index);
        result.push(char);
    }
    return result;
}

// Interesting loop syntax
fn loop_till(n: i32) -> i32 {
    let mut i = 1;
    let result = loop {
        if i == n {
            break i * 12;
        }
        i += 1;
    };
    println!("Done!");
    return result;
}