use std::{fs, usize};
use regex::Regex;

fn hash_alg(s: &str) -> u32 {

    let mut current_value: u32 = 0;
    for c in s.chars() {
        if c == '\n' { continue; }
        //println!("Char [{}]", c);
        let ascii_value = c as u8;
        current_value += ascii_value as u32;
        current_value *= 17;
        current_value = current_value % 256;
    }
    return current_value;

}

struct Lens {
    label: String,
    strength: u32
}

impl Lens {
    fn new(label: String, strength: u32) -> Lens {
        Lens {
            label,
            strength
        }
    }
}

fn main() {

    //let file_path = "test_input.txt";
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    let segments: Vec<&str> = contents.split(',').collect();

    for segment in segments {
        //println!("Seg: {}", segment.trim_end().trim_end_matches('\n'));
        let re = Regex::new(r"[-=]").unwrap();
        let symbol_index = re.find(segment).unwrap().start();
        let (label, focal_length) = segment.split_at(symbol_index);
        let box_no = hash_alg(label);

        //println!("Line: {}", segment);
        //println!("Box: {}", box_no);
        //println!("Symbol: {}", focal_length.chars().nth(0).unwrap());
        if focal_length.chars().nth(0).unwrap() == '-' {
            let mut found_lens_index = 999;
            for (i, lens) in boxes.get(box_no as usize).unwrap().iter().enumerate() {
                if lens.label == label {
                    found_lens_index = i;
                    break;
                }
            }
            if found_lens_index != 999 {
                //println!("Found lens to pop: Box {} label {}", box_no, label);
                boxes.get_mut(box_no as usize).unwrap().remove(found_lens_index);
            }
        } else {
            let (_, focal_length) = focal_length.split_once('=').unwrap();

            let mut found_lens_index = 999;
            for (i, lens) in boxes.get(box_no as usize).unwrap().iter().enumerate() {
                if lens.label == label {
                    found_lens_index = i;
                    break;
                }
            }

            let new_lens = Lens::new(label.to_string(), focal_length.trim_end_matches("\n").parse().unwrap());
            if found_lens_index != 999 {
                //println!("Replaced lens");
                boxes.get_mut(box_no as usize).unwrap()[found_lens_index] = new_lens;
            } else {
                boxes.get_mut(box_no as usize).unwrap().push(new_lens);
                //println!("Added new lens");
            }

        }
        //println!("");
        
    }

    let mut total = 0;
    for (j, lens_box) in boxes.iter().enumerate() {
        for (i, lens) in lens_box.iter().enumerate() {
            let value = (1 + j as u32) * (1 + i as u32) * lens.strength;
            //println!("Box: {}", j);
            //println!("Slot: {}", i);
            //println!("Label: {}", value);
            //println!("Value: {}", value);
            total += value;
        }
    }

    println!("Total: {}", total)

}
