use std::fs;

fn main() {

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut total = 0;
    const TWO: u32 = 2;

    for card in contents.lines() {

        let (game_str, all_numbers_str) = card.split_once(":").unwrap();
        let (winning_numbers_str, card_numbers_str) = all_numbers_str.split_once("|").unwrap(); 

        let game_number = game_str.chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .to_string()
            .parse::<u32>().unwrap();
        let winning_numbers: Vec<u32> = winning_numbers_str.split_whitespace()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>()
                 .unwrap()).collect();
        let card_numbers: Vec<u32> = card_numbers_str.split_whitespace()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>()
                 .unwrap()).collect();

        let mut card_value = 0;
        let mut power = 0;
        println!("Game: {} start", game_number);

        // Match winners and update card value
        for num in card_numbers {
            if winning_numbers.contains(&num) {
                card_value = TWO.pow(power);
                power += 1;
                println!("Card: {}", card_value);
            }
        }


        // Add card value
        
        println!("Game: {} over", game_number);
        println!("Card value: {}", card_value);
        total += card_value;
        
        
    }

    println!("Total: {}", total);

}
