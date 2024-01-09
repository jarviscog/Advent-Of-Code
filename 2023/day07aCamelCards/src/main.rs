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
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {

    fn from_char(c: char) -> Option<Card> {
        match c {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'J' => Some(Card::Jack),
            'T' => Some(Card::Ten),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            _ => None
        }
    }

    fn to_char(&self) -> char {
        match self {
             Card::Ace => 'A',
             Card::King => 'K',
             Card::Queen => 'Q',
             Card::Jack => 'J',
             Card::Ten => 'T',
             Card::Nine => '9',
             Card::Eight => '8',
             Card::Seven => '7',
             Card::Six => '6',
             Card::Five => '5',
             Card::Four => '4',
             Card::Three => '3',
             Card::Two => '2',
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
             Card::Jack => 9,
             Card::Ten => 8,
             Card::Nine => 7,
             Card::Eight => 6,
             Card::Seven => 5,
             Card::Six => 4,
             Card::Five => 3,
             Card::Four => 2,
             Card::Three => 1,
             Card::Two => 0,
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

    // This could be better
    fn card_bits(&self) -> u64 {
        const TEN: u64 = 10;
        let mut ans = 0;
        for card in self.cards {
            ans += TEN.pow(card.value());
        }
        //println!("Card bits: {}", ans);
        ans

    }

    fn hand_as_value(&self) -> u32 {
        let mut ans: u32 = 0;
        ans += self.cards[4].value() << 0;
        ans += self.cards[3].value() << 4;
        ans += self.cards[2].value() << 8;
        ans += self.cards[1].value() << 12;
        ans += self.cards[0].value() << 16;
        ans
    }

    fn is_five_of_kind(bits: u64) -> bool {
        if bits.to_string().contains("5") {
            return true;
        }
        false
    }

    fn is_four_of_kind(bits: u64) -> bool {
        if bits.to_string().contains("4") {
            return true;
        }
        false
    }

    fn is_full_house(bits: u64) -> bool {
        if bits.to_string().contains("3") && bits.to_string().contains("2") {
            return true;
        }
        false
    }

    fn is_three_of_kind(bits: u64) -> bool {
        if bits.to_string().contains("3") {
            return true;
        }
        false
    }

    fn two_pair(bits: u64) -> bool {
        let re = Regex::new(r"2").unwrap();
        if re.find_iter(&bits.to_string()).count() == 2 {
            return true;
        }
        false
    }

    fn one_pair(bits: u64) -> bool {
        if bits.to_string().contains("2") {
            return true;
        }
        false
    }

    fn type_value(&self) -> u32 {
        let bits = self.card_bits();
        if Hand::is_five_of_kind(bits) { return 6 } 
        if Hand::is_four_of_kind(bits) { return 5 } 
        if Hand::is_full_house(bits) { return 4 } 
        if Hand::is_three_of_kind(bits) { return 3 } 
        if Hand::two_pair(bits) { return 2 } 
        if Hand::one_pair(bits) { return 1 } 
        0
    }

    fn ultimate_value(&self) -> u32 {
        // Bits 0-15 are the ultimate hand value, bits 16-18 are for the type value
        let mut ans = 0;
        ans += self.hand_as_value();
        ans += self.type_value() << 20;
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
    fn to_string(&self) -> String {
        format!("Hand: {} With bet: {}", self.hand.to_string(), self.bet)
    }
}

fn main() {
    
    let t1 = Instant::now();

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
    
    println!("Time: {:.2?}", t1.elapsed());
    let t1 = Instant::now();

    // Now this is just a sorting question, and we can have rust do it for us
    let mut sum = 0;
    hand_bet_pairs.sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());
    for (i, hb) in hand_bet_pairs.iter().enumerate() {
        println!("{}, {:15} ", hb.hand.to_string(), hb.hand.card_bits());
        //println!("Bet: {}, Rank: {} Winnings: {}", hb.bet, (i + 1), winnings);
        sum += hb.bet as usize * (i + 1);
    }
    println!("{}", sum);
    assert_eq!(sum, 253603890);
    println!("Time: {:.2?}", t1.elapsed());
}
