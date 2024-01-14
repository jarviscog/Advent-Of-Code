use std::{fs, usize};
use regex::Regex;


fn get_possibilities(springs: &str, counts: Vec<u8>) -> u32 {

    //springs = springs.trim_end_matches(".");
    print!("New call: {} ", springs);
    for n in &counts {
        print!(" {}", n);
    }
    println!("");

    let next_spring_location = springs.find("#");
    let next_dot_location = springs.find(".");
    let next_question_location = springs.find("?");

    if springs.len() <= 1 {
        println!("Length of 1");
        return 1;
    }

    if next_question_location.is_none() {
        println!("No more question marks");
        return 1;
    }


    // If the first symbol is a dot, get rid of it and go again
    if let Some(loc) = next_dot_location {
        if loc == 0 {
            println!("First is dot");
            return get_possibilities(&springs[1..springs.len()], counts);
        }
    }

    // If there is only one number left
    if counts.len() == 1 {
        println!("Only one number left");
        if springs.len() == counts[0] as usize {
            return 1;
        }
        // If only question marks
        if !springs.contains("#") && !springs.contains(".") {
            let possibilities = springs.len() + 1 - counts[0] as usize;
            println!("Possibilities: {}", possibilities);
            return possibilities as u32;
        } else if !springs.contains(".") { // There is only ??? and ### 
            // Find the first and last ???. This will be our bounds
            let first_position = springs.find("?").unwrap();
            let mut last_position = 0;
            for (i, c) in springs.chars().enumerate() {
                if c == '?' {
                    last_position = i;
                }

            }

            let min_width = last_position - first_position + 1;
            let left_clipping = 0;
            let right_clipping = 0;

            

        }



    }

    if let Some(loc) = next_spring_location {

        // If the first symbol is a spring, fit it to the next number
        if loc == 0 {
            println!("First is hash");
            if springs.len() == counts[0] as usize {
                return 1;
            } else {
                return get_possibilities(&springs[(counts[0] + 1) as usize..], counts[1..].to_vec()); // + 1 for the . gap needed
            }
        }
        // If there is a # at the front
        else if loc < next_question_location.unwrap() {
            let length_of_first_spring: usize = counts[0] as usize;
            return get_possibilities(&springs[length_of_first_spring..], counts)
        }
    }

    // Remove a block from the end that has been solved e.g. ?????????..### 2, 1, 3
    let ending_solved_hash_block_re = Regex::new(r"\.+(#+)\.*$").unwrap();
    let ending_block = ending_solved_hash_block_re.find(&springs);
    if let Some(block) = ending_block {
        println!("Removing solved block from end: {}", block.as_str());
        return get_possibilities(&springs[0..block.start()], counts[0..counts.len()- 1].to_vec());
    }

    if !springs.contains("#") && !springs.contains(".") {
        println!("Only question marks");
        for num in &counts {
            let marks_to_replace = (0..*num).map(|_| "?").collect::<String>();
            let temp_springs = &springs.replacen(&marks_to_replace, "?", 1);
            println!("Working springs: {}", temp_springs);
        }

    }

    if let Some(loc) = next_question_location {

        let question_re = Regex::new(r"\?*").unwrap();
        let spring_re = Regex::new(r"(\#+)").unwrap();
        let size_of_question_block = question_re.find(&springs).unwrap().len();
        let size_of_next_hash_block = spring_re.find(&springs);

        if loc == 0 {
            
            // If the pattern goes ?# the next segment is a part of that block
            if size_of_question_block == 1 {

                // If that block is the same size as our piece e.g. ?###..## 3,2
                if let Some(hash_block_length) = size_of_next_hash_block {
                    if hash_block_length.len() as u8 == counts[0] {
                        println!("?# pattern. New springs: {}", &springs[(hash_block_length.len() + 2)..]);
                        return get_possibilities(&springs[(hash_block_length.len() + 2)..], counts[1..].to_vec())
                    }
                }
            }

            // If we can match the number of ???? to the next length of springs
            //if counts[0] as usize == size_of_question_block {
            //    println!("");
            //    return get_possibilities(&springs[size_of_question_block..], counts[1..].to_vec());
            //} else if counts[0] as usize + 1 == size_of_question_block {
            //    return 2 * get_possibilities(&springs[size_of_question_block..], counts[1..].to_vec());
            //}

        }
    }

    if counts.len() > 1 {
        let next_question_hash_re = Regex::new(r"^([\?\#])+.").unwrap();
        let first_question_hash_segment = next_question_hash_re.find(springs);
        // If only the first number fits in the next segment seperated by a .
        if (counts[0] + counts[1] ) as usize > first_question_hash_segment.unwrap().len() - 1 {
            println!("Only the first one will fit in this segment");
            return get_possibilities(&springs[0..first_question_hash_segment.unwrap().len() - 1], counts[0..1].to_vec())
                * get_possibilities(&springs[first_question_hash_segment.unwrap().len()..], counts[1..].to_vec());

        }
    }

    print!("============= I don't know what to do here sorry Jarvis: {} ", springs);
    for n in counts {
        print!(" {}", n);
    }
    println!(" ======");

    1

}

fn main() {

    let file_path = "input.txt";
    //let file_path = "test_two_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut total_arrangements = 0;
    for line in contents.lines() {

        println!("Line: [{}]", line);

        let (springs_raw, counts_raw) = line.split_once(" ").unwrap();
        let mut springs: String = springs_raw.replace(r"\.*", ".");
        let counts: Vec<u8> = counts_raw.split(",").map(|n| n.parse().unwrap()).collect();
        
        springs = springs.trim_matches('.').to_string();
        let line_total = get_possibilities(&springs, counts);
        println!("Line total: {}\n", line_total);
        total_arrangements += line_total;
        println!("\n");

    }

    println!("Total Cases: {}\n", total_arrangements);
}
