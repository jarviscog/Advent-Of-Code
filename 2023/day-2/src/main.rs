use std::fs;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn is_round_possible(red: u32, green: u32, blue:u32) -> bool {

    println!("{red}");
    println!("{green}");
    println!("{blue}");
    if red > MAX_RED {return false};
    if green > MAX_GREEN {return false};
    if blue > MAX_BLUE {return false};
    
    true
}


fn main() {

    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");


    let mut game_id_sum = 0;
    let mut total_power_sum = 0;
    // Iterate games
    for line in contents.lines() {
        let mut is_game_valid = true;
        println!("");
        let (game_id_str, rounds_str) = line.split_once(":").unwrap();
        let game_id: u32 = game_id_str.replace("Game ", "").parse::<u32>().unwrap();
        let rounds = rounds_str.split(";");
        
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        

        println!("{game_id}");
        for round in rounds {
            println!("{round}");

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for color in round.split(",") {

                // This is so satisfying
                let color_value = color.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap_or(0);

                if color.contains("red") { 
                    red = color_value; 
                    if red > min_red { min_red = red; }
                }
                if color.contains("green") { 
                    green = color_value; 
                    if green > min_green { min_green = green; }
                }
                if color.contains("blue") { 
                    blue = color_value; 
                    if blue > min_blue { min_blue = blue; }
                }

            }

            if !is_round_possible(red, green, blue) { is_game_valid = false; }

        }
        if is_game_valid {
            game_id_sum += game_id;
        }
        let power = min_red * min_green * min_blue;
        total_power_sum += power;
    }

    println!("{game_id_sum}");
    println!("Power sum: {total_power_sum}");


}
