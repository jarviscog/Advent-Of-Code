use std::fs;
use regex::Regex;
use std::cmp::Ordering;
use std::time::Instant;

// This problem seems like a great one to get into ideas of enums and strucs, so I'm going to do
// some things that don't make sense for this problem, but are good to know


#[derive(Copy, Clone)]
enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {

    fn from_char(c: char) -> Option<Card> {
        match c {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'T' => Some(Card::Ten),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            'J' => Some(Card::Joker),
            _ => None
        }
    }

    fn to_char(&self) -> char {
        match self {
             Card::Ace => 'A',
             Card::King => 'K',
             Card::Queen => 'Q',
             Card::Ten => 'T',
             Card::Nine => '9',
             Card::Eight => '8',
             Card::Seven => '7',
             Card::Six => '6',
             Card::Five => '5',
             Card::Four => '4',
             Card::Three => '3',
             Card::Two => '2',
             Card::Joker => 'J',
        }
    }

    fn to_string(&self) -> String {
        String::from(self.to_char())
    }

    fn value(&self) -> u32 {
        match self {
             Card::Ace => 12,
             Card::King => 11,
             Card::Queen => 10,
             Card::Ten => 9,
             Card::Nine => 8,
             Card::Eight => 7,
             Card::Seven => 6,
             Card::Six => 5,
             Card::Five => 4,
             Card::Four => 3,
             Card::Three => 2,
             Card::Two => 1,
             Card::Joker => 0,
        }
    }

}

struct Hand {
    cards: [Card; 5]
}

impl Hand {
    
    fn new(cards: [Card; 5]) -> Hand {
        Hand {
            cards
        }
    }

    fn to_string(&self) -> String {
        format!("{}{}{}{}{}", 
                self.cards[0].to_char(),
                self.cards[1].to_char(),
                self.cards[2].to_char(),
                self.cards[3].to_char(),
                self.cards[4].to_char()
               )
    }

    // Returns a string where each column is the number of each card, with the ones column being
    // the twos, the tens colum being the threes, etc.
    // So 23557 is "102011"
    fn hand_as_string(&self) -> String {
        const TEN: u64 = 10;
        let mut ans = 0;
        for card in self.cards {
            if card.to_char() != 'J' {
                ans += TEN.pow(card.value());
            }
        }
        ans.to_string()
    }

    fn hand_type_hash(&self) -> u32 {
        let mut digits: Vec<u32> = self.hand_as_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
        digits.sort();
        let mut ans = 0;
        let mut place_loc = 0;
        while let Some(d) = digits.pop() {
            if d == 0 {
                break;
            }
            ans += d << place_loc;
            place_loc += 4;
        }
        return ans
    }

    // if hash == 0x00014 { return 5 } // XXXXY -> 4 of kind
    // if hash == 0x00012 { return 5 } // XXYJJ -> 4 of kind 
    // if hash == 0x00011 { return 5 } // XYJJJ -> 4 of kind 
    // if hash == 0x00013 { return 5 } // XXXYJ -> 4 of kind
    // if hash == 0x00023 { return 4 } 
    // if hash == 0x00022 { return 4 } // XXYYJ -> Full House
    // if hash == 0x00113 { return 3 }
    // if hash == 0x00112 { return 3 } // XXYZJ -> Three of kind
    // if hash == 0x00111 { return 3 } // XYZJJ -> Three of kind 
    // if hash == 0x00122 { return 2 }
    // if hash == 0x01112 { return 1 }
    // if hash == 0x01111 { return 1 } // XYZNJ -> One pair
    // if hash == 0x11111 { return 0 }
    fn type_value(&self) -> u32 {
        let hash = self.hand_type_hash();
        if hash < 0x00010 { return 6 }
        if hash < 0x00020 { return 5 }
        if hash < 0x00030 { return 4 }
        if hash < 0x00120 { return 3 }
        if hash < 0x00130 { return 2 }
        if hash < 0x10000 { return 1 }
        return 0;
    }

    fn ultimate_value(&self) -> u32 {
        // Bits 0-15 are the ultimate hand value, bits 24-27 are for the type value
        let mut ans = 0;
        ans += self.cards[4].value() << 0;
        ans += self.cards[3].value() << 4;
        ans += self.cards[2].value() << 8;
        ans += self.cards[1].value() << 12;
        ans += self.cards[0].value() << 16;
        ans += self.type_value() << 24;
        ans
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.ultimate_value() == other.ultimate_value()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.ultimate_value() > other.ultimate_value() {
            Some(Ordering::Greater)
        } else if self.ultimate_value() < other.ultimate_value() {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

struct HandBet {
    hand: Hand,
    bet: u32
}

impl HandBet {
    fn new(hand: Hand, bet: u32) -> HandBet {
        HandBet {
            hand,
            bet
        }
    }
}

fn main() {
    
    let now = Instant::now();

    let file_path = "input.txt";
    //let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    // Put the cards into a list
    let mut hand_bet_pairs: Vec<HandBet> = Vec::new();
    for line in contents.lines() {
        let (cards_str, bet_str) = line.split_once(" ").unwrap();
        let cards_chars = cards_str.chars();
        let mut cards: [Card; 5] = [Card::Ace; 5];
        for (i, card_char) in cards_chars.enumerate() {
            cards[i] = Card::from_char(card_char).unwrap();
        }
        let bet: u32 = bet_str.parse::<u32>().unwrap();
        hand_bet_pairs.push(HandBet::new(Hand::new(cards), bet));
    }
    
    // Now this is just a sorting question, and we can have rust do it for us
    let mut sum = 0;
    hand_bet_pairs.sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());
    for (i, hb) in hand_bet_pairs.iter().enumerate() {
        let winnings = hb.bet as usize * (i + 1);
        //println!("{}, {}, {:X}, {}", hb.hand.to_string(), hb.bet, hb.hand.hand_type_hash(), hb.hand.type_value());
        sum += winnings;
    }
    println!("Finished in {:.2?}", now.elapsed());
    println!("{}", sum);
}
