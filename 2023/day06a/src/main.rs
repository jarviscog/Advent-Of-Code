use std::fs;



fn main() {

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Coud not read file");

    for line in contents.lines() {

        println!("{}", line);
    }
}
