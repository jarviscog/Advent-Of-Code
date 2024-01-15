use std::fs;

fn main() {

    //let file_path = "test_input.txt";
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut segments: Vec<&str> = Vec::new();
    segments = contents.split(',').collect();

    let mut total = 0;
    for segment in segments {

        println!("Seg: {}", segment.trim_end().trim_end_matches('\n'));
        let mut current_value: u32 = 0;
        for c in segment.chars() {
            if c == '\n' { continue; }
            println!("Char [{}]", c);
            let ascii_value = c as u8;
            current_value += ascii_value as u32;
            current_value *= 17;
            current_value = current_value % 256;
        }
        println!("{}", current_value);
        total += current_value;
    }

    println!("Total: {}", total)

}
