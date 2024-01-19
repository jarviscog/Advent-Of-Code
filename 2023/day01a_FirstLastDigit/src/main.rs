use std::fs;

fn main() {

    // Load file
    let file_path: &str = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Could not access file");

    // Iterate lines
    let mut total_sum = 0;

    for line in contents.lines() {

        let s: String = line.chars().filter(|c| c.is_numeric()).collect();
        let first: u32 = s.chars().rev().last().unwrap().to_digit(10).unwrap() * 10;
        let last: u32 = s.chars().last().unwrap().to_digit(10).unwrap();

        let final_num = first + last;
        total_sum += final_num;

    }
    println!("{total_sum}");
    
}
