use std::fs;

fn main() {

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut cards: Vec<u32> = Vec::new();

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

        let mut number_of_wins = 0;
        cards.push(game_number);

        // Match winners and update card value
        for num in card_numbers {
            if winning_numbers.contains(&num) {
                number_of_wins += 1;
            }
        }


        let number_of_nth_card = cards.iter().filter(|n| game_number.eq(n)).count();
        println!("I have {} cards for game {} that won {} times each.", number_of_nth_card, game_number, number_of_wins);
        for i in 0..number_of_nth_card {
            
            let mut temp_card_num = game_number + 1;
            let mut temp_number_of_wins = number_of_wins;
            while temp_number_of_wins > 0 {
                //println!("Pushing {temp_card_num}");
                cards.push(temp_card_num);
                temp_card_num += 1;
                temp_number_of_wins -= 1;
            }
        }

        
        
        
    }

    println!("Number of cards: {}", cards.iter().count());

}
