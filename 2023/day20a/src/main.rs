use std::fs;

fn main() {

    //let file_path = "input.txt";
    let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");
    println!("Hello, world!");
}
