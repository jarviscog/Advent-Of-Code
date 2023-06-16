use std::env;
use std::fs;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file: {}", file_path);

    //let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    
    let mut calories = Vec::new();
    let mut current_running_total: i32 = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {

        let mut current_num = 0;
        match line.to_string().parse::<i32>() {
            Ok(n) => current_num = n,
            Err(_) => (),
        }
        current_running_total += current_num;

        if line == "" {
            calories.push(current_running_total);
            current_running_total = 0;
        }
    }
   
    calories.sort();
    calories.reverse();
    println!("Top: {}", calories[0]);
    let total: i32 = calories[0] + calories[1] + calories[2];
    println!("Total: {}", total);

}
