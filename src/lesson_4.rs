use crate::lesson::Lesson;

pub struct Lesson4 { pub name: String }

impl Lesson for Lesson4 {
    fn run(&self) -> () {
        println!("\n{}:", self.name);

        let n_s: String;
        let full_copy_s: String;
        {
            let s = String::from("Hello");
            println!("The string {}", s);

            // ***************** Deep Copy
            full_copy_s = s.clone();

            // ***************** Shallow copy
            // Rust only allows 1 owner of a value at a time; therefore this is called a MOVE
            n_s = s;
            // Compiler complains cause value was moved
            // println!("The string s is now {}", s); --> DOES NOT WORK
            // However this will work since we copy the values into another space on the heap
            println!("The string s is now {}", full_copy_s);
        }
        println!("Yet the string here is now {}", n_s);

        // References
        let s = String::from("Test string hello");
        let first_word = get_first_word(&s);
        println!("The first word is: {}", first_word);

        // CHALLENGE TRIM SPACES:
        println!("\n\nCHALLENGE:");
        let test1 = "We need more space.";
        assert_eq!(trim_spaces(test1), "We need more space.");
        
        let test2 = String::from("   There's space in front.");
        assert_eq!(trim_spaces(&test2), "There's space in front.");
        
        let test3 = String::from("There's space to the rear. ");
        assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
        
        let test4 = "  We're surrounded by space!    ";
        assert_eq!(trim_spaces(test4), "We're surrounded by space!");
        
        let test5 = "     ";
        assert_eq!(trim_spaces(test5), "");
        
        let test6 = "";
        assert_eq!(trim_spaces(test6), "");
        
        let test7 = " 🚀 ";
        assert_eq!(trim_spaces(test7), "🚀");
        println!("Tests passed!");
    }
}

fn trim_spaces(s: &str) -> &str {
    if s.len() == 0 {
        return &s;
    }

    let mut result = s;

    while *result.as_bytes().first().unwrap() == b' ' {
        result = &result[1..];
        if result.len() == 1 && *result.as_bytes().first().unwrap() == b' ' {
            return "";
        }
    }

    while *result.as_bytes().last().unwrap() == b' ' {
        let end_index = result.len() - 1;
        result = &result[..end_index];
        if result.len() == 1 && *result.as_bytes().last().unwrap() == b' ' {
            return "";
        }
    }

    return &result;
}

// When writting string functions, best practice to use str data type cause it can be used for str or Sring datatypes
fn get_first_word(str: &str) -> &str {
    let string_bytes = str.as_bytes();
    println!("{:?}", string_bytes);

    for (index, &item) in string_bytes.iter().enumerate() {
        if item == b' ' {
            let res = &str[..index];
            println!("Found first word! -> {} <-", res);
            return res;
        }
    }

    println!("No space bytes found");
    return &str;
}