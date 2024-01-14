use std::fs;
use std::str;

fn solve_reflection(row_values: Vec<u32>, column_values: Vec<u32>) -> u32 {
//    for n in &row_values {
//        println!("Horozontal: {}", n);
//    }
//    for n in &column_values {
//        println!("Vertical: {}", n);
//    }
//    println!();

    
    let height = row_values.len();
    for y in 0..height {
        let mut i: i32 = 0; 
        let mut symmetric = false;
        while y as i32 - i >= 0 && y as i32 + 1 + i < height as i32 {
            let left = row_values.get(y - i as usize);
            let right = row_values.get((y + 1) + i as usize);

            println!("y: {}, I: {}", y, i);
            if left != right {
                println!("No match");
                symmetric = false;
                break;
            } else {
                symmetric = true;
            }
            
            i += 1;
        }
        if symmetric == true {
            println!("Symmetry at Y: {}", y + 1);
            return 100 * (y + 1) as u32;
        }
    }

    let width = column_values.len();
    for x in 0..width {
        let mut i: i32 = 0; 
        let mut symmetric = false;
        while x as i32 - i >= 0 && x as i32 + 1 + i < width as i32 {
            let left = column_values.get(x - i as usize);
            let right = column_values.get((x + 1) + i as usize);
            println!("x: {}, I: {}", x, i);
            if left != right {
                symmetric = false;
                println!("No match");
                break;
            } else {
                symmetric = true;
            }
            i += 1;
        }
        if symmetric == true {
            println!("Symmetry at X: {}", ((x + 1) as u32));
            return (x + 1) as u32;
        }
    }
    panic!("No symmetry found");
}

fn main() {

    let file_path = "input.txt";
    //let file_path = "test_input.txt";
    let mut contents = fs::read_to_string(file_path)
        .expect("Could not read file");
    contents.push('\n');

    let mut horozontal_values: Vec<u32> = Vec::new();
    let mut vertical_lines: Vec<Vec<char>> = Vec::new();
    let mut vertical_values: Vec<u32> = Vec::new();
    let mut total = 0;
    for line in contents.lines() {
        println!("Line: [{}]", line);
        if line.is_empty() {

            //println!("End of block");
            for line in vertical_lines {
                let mapped_value_vec: String = line.into_iter().collect::<String>().chars()
                    .map(|c| 
                         match c {
                             '.' => '0',
                             '#' => '1',
                             _ => panic!("Unexpected symbol in pattern")
                         }
                        ).collect();
                let mapped_value = u32::from_str_radix(&mapped_value_vec, 2).unwrap();
                vertical_values.push(mapped_value);
            }

            total += solve_reflection(horozontal_values, vertical_values);
            horozontal_values = Vec::new();
            vertical_values = Vec::new();
            vertical_lines = Vec::new();
            continue;
        } 

        if horozontal_values.is_empty() && !line.is_empty() {
            //println!("First in line");
            for _ in 0..line.len() {
                vertical_lines.push(Vec::new());
            }
            //println!("Width of this block: {}", vertical_lines.len());
        }

        for (j, c) in line.chars().enumerate() {
            vertical_lines[j].push(c);
        }

        let mapped_value_vec: String = line.chars()
            .map(|c| 
                 match c {
                     '.' => '0',
                     '#' => '1',
                     _ => panic!("Unexpected symbol in pattern")
                 }
                ).collect();
        let mapped_value = u32::from_str_radix(&mapped_value_vec, 2).unwrap();
        //println!("{}", mapped_value);
        horozontal_values.push(mapped_value)
        
    }
    println!("Total: {}", total);
}
