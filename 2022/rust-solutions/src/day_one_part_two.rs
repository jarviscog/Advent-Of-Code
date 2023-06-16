use std::env;
use std::fs;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file: {}", file_path);

    //let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    
    let mut current_max: i32 = 0;
    let mut current_running_total: i32 = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {

        let mut current_num = 0;
        match line.to_string().parse::<i32>() {
            Ok(n) => current_num = n,
            Err(_) => (),
        }
        current_running_total += current_num;

        if line == "" {
//            println!("Total: {}", current_running_total);
            if current_running_total > current_max {
                current_max = current_running_total;
            }
            current_running_total = 0;
        }
    }

    println!("Largest: {} ", current_max);


}
