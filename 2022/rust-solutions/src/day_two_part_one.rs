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
                let opp = ip.as_bytes()[0] as char;
                let you = ip.as_bytes()[2] as char;                
                //println!("{}", values[&you]);    
                sum += values[&you];
                sum += outcome(values[&you], values[&opp]);
            }
            
        }
    }

    println!("{}", sum);

}
