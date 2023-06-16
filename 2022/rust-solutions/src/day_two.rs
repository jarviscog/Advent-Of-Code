use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
fn read_lines<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>>
where P:AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())

}
fn outcome(you:i32, opp:i32) -> i32{
    if you == opp {return 3} // Tie game
    if you == 1 && opp == 3 {return 6} // Rock vs siscors 
    if you == 3 && opp == 1 {return 0} // siscors vs rock
    if you > opp {return 6}
    0
}
fn main() {
    


    let values = HashMap::from([
                               ('A', 1),
                               ('B', 2),
                               ('C', 3),
                               ('X', 1),
                               ('Y', 2),
                               ('Z', 3),
    ]);


    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file: {}", file_path);
    let mut sum = 0;
    if let Ok(lines) = read_lines(file_path){
        for line in lines {
            if let Ok(ip) = line {

                let opp_char = ip.as_bytes()[0] as char;
                let you_char = ip.as_bytes()[2] as char;                
                let you_val = values[&you_char];
                let opp_val = values[&opp_char];

                let your_move_val: i32;
                // Convert you to actual move
                match you_val {
                    1 => {your_move_val = ((opp_val + 1) % 3) + 1},
                    2 => {your_move_val = opp_val},
                    3 => {your_move_val = (opp_val % 3) + 1} 
                    _ => {your_move_val = 0}
                }
                
                println!("{}", your_move_val);
                sum += your_move_val;
                sum += outcome(your_move_val, opp_val);
            }
            
        }
    }

    println!("{}", sum);

}
