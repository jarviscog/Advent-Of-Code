use std::fs;
use regex::Regex;

fn main() {

    // Load file
    let file_path: &str = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Could not access file");


    // Iterate lines
    
    let re = Regex::new(r"[A-Za-z]").unwrap();
    let mut total_sum = 0;

    for line in contents.lines() {

        let mut edited_string: String = String::from(line);

        println!("{edited_string}");
        // === Part 2 === 
        // This is an extremely janky solution, but funny
        // This is needed because sometimes numbers overlap (e.g. oneight), 
        // and I was to lazy to fix my regex solution
        edited_string = edited_string.replace("one", "o1e");
        edited_string = edited_string.replace("two", "t2o");
        edited_string = edited_string.replace("three", "t3e");
        edited_string = edited_string.replace("four", "f4r");
        edited_string = edited_string.replace("five", "f5e");
        edited_string = edited_string.replace("six", "s6x");
        edited_string = edited_string.replace("seven", "s7n");
        edited_string = edited_string.replace("eight", "e8t");
        edited_string = edited_string.replace("nine", "n9e");
        
        edited_string = edited_string.replace("one", "o1e");
        edited_string = edited_string.replace("two", "t2o");
        edited_string = edited_string.replace("three", "t3e");
        edited_string = edited_string.replace("four", "f4r");
        edited_string = edited_string.replace("five", "f5e");
        edited_string = edited_string.replace("six", "s6x");
        edited_string = edited_string.replace("seven", "s7n");
        edited_string = edited_string.replace("eight", "e8t");
        edited_string = edited_string.replace("nine", "n9e");

        let s = re.replace_all(&edited_string[..], "");

        println!("{s}");

        let first: u32 = s.chars().rev().last().unwrap().to_digit(10).unwrap() * 10;
        let last: u32 = s.chars().last().unwrap().to_digit(10).unwrap();

        let final_num = first + last;
        total_sum += final_num;
        println!("\n");

    }

    println!("{total_sum}");
    
    //println!("Hello, {contents}!");
}
