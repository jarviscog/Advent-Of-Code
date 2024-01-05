use std::{fs, usize};
use std::collections::HashMap;


const WIDTH: usize = 140;
const HEIGHT: usize = 140;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub value: String,
}

impl Point {

    fn new(x: u32, y: u32, value: String) -> Point {
        Point{
            x,
            y,
            value,
        }
    }

    fn to_string(&self) -> String {
        return format!("({}, {}) {} ", self.x, self.y, self.value); 
    }

}

fn print_grid(grid: [[char; WIDTH] ; HEIGHT]) {
    for row in grid {
        for element in row {
            print!("{element}");
        }
        println!("");
    }
}

fn same_location(p1: &Point, p2: &Point) -> bool {
    
    if p1.x != p2.x { return false };
    if p1.y != p2.y { return false };

    true
}

fn adjacent_symbol(symbols: Vec<Point>, p: Point) -> Option<Point> {

    // This function is terrible. I should fix this at some point
    let left: Point;
    let right: Point;
    let top: Point;
    let bottom: Point;
    let top_right: Point;
    let top_left: Point;
    let bottom_right: Point;
    let bottom_left: Point;
    
    if p.x > 0 {
        left = Point::new(p.x-1, p.y, String::from("."));
    } else {
        left = p.clone();
    }
    if p.y > 0 {
        top = Point::new(p.x, p.y-1, String::from("."));
    } else {
        top = p.clone();
    }
    if p.x < WIDTH.try_into().unwrap() {
        right = Point::new(p.x+1, p.y, String::from("."));
    } else {
        right = p.clone();
    }
    if p.y < HEIGHT.try_into().unwrap() {
        bottom = Point::new(p.x, p.y+1, String::from("."));
    } else {
        bottom = p.clone();
    }

    if p.x > 0 && p.y > 0{
        top_left = Point::new(p.x-1, p.y-1, String::from("."));
    } else {
        top_left = p.clone();
    }
    if p.x > 0 && p.y < HEIGHT.try_into().unwrap(){
        bottom_left = Point::new(p.x-1, p.y+1, String::from("."));
    } else {
        bottom_left = p.clone();
    }
    if p.x < WIDTH.try_into().unwrap() && p.y > 0 {
        top_right = Point::new(p.x+1, p.y-1, String::from("."));
    } else {
        top_right = p.clone();
    }
    if p.x < WIDTH.try_into().unwrap() && p.y < HEIGHT.try_into().unwrap(){
        bottom_right = Point::new(p.x+1, p.y+1, String::from("."));
    } else {
        bottom_right = p.clone();
    }

    for symbol in symbols {
        
        if symbol.value == String::from("*") { // For part 2

            if same_location(&symbol, &top_left) { return Some(symbol) }
            if same_location(&symbol, &top) { return Some(symbol) }
            if same_location(&symbol, &top_right) { return Some(symbol) }
            if same_location(&symbol, &right) { return Some(symbol) }
            if same_location(&symbol, &left) { return Some(symbol) }
            if same_location(&symbol, &bottom_left) { return Some(symbol) }
            if same_location(&symbol, &bottom) { return Some(symbol) }
            if same_location(&symbol, &bottom_right) { return Some(symbol) }

        }

    }

    None 

}

fn main() {

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut grid = [['0' as char; WIDTH] ; HEIGHT];
    let mut symbols: Vec<Point> = Vec::new();
    let mut numbers: Vec<Point> = Vec::new();

    // Place elements in a grid
    for (i, line) in contents.lines().into_iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }
    
    // Collect all number and symbol locations
    for (j, row) in grid.iter().enumerate() {
        let mut fresh_number = true;
        let mut fresh_number_x = 0;
        for (i, el) in row.iter().enumerate() {

            if el.is_ascii_digit() && fresh_number == true {
                    fresh_number = false;
                    fresh_number_x = i;
            }

            if el.is_ascii_punctuation()  {

                if fresh_number == false {
                    let num: String = String::from_iter(row.clone()[fresh_number_x..i].iter());
                    numbers.push(Point::new(fresh_number_x.try_into().unwrap(), j.try_into().unwrap(), num));
                    fresh_number_x = 0;
                    fresh_number = true; 
                }

                if !(el == &'.') {
                    let x: u32 = i.try_into().unwrap();
                    let y: u32 = j.try_into().unwrap();
                    let p = Point::new(x, y, String::from(*el));
                    symbols.push(p.clone());
                    fresh_number = true;
                } 

            }
        }
        if fresh_number == false {
            let num: String = String::from_iter(row.clone()[fresh_number_x..WIDTH].iter());
            numbers.push(Point::new(fresh_number_x.try_into().unwrap(), j.try_into().unwrap(), num));
        }
    }

    let mut sum = 0;
    let mut gear_sum = 0;
    let mut gear_map: HashMap<Point, Point> = HashMap::new();
    for number in numbers {

        println!("Number: {}", number.to_string());

        let mut current_x = number.x;
        for subnum in number.value.chars() {
            //println!("{}", subnum);
            let n = Point::new(current_x, number.y, String::from(subnum));
            if let Some(s) = adjacent_symbol(symbols.clone(), n) {
                //println!("Connection: {}", subnum.to_string());
                println!("Connected: {}", number.to_string());
                sum += number.value.parse::<u32>().unwrap();
                if gear_map.contains_key(&s) {
                    let n = number.value.parse::<u32>().unwrap();
                    let m = gear_map.get(&s).unwrap().value.parse::<u32>().unwrap();
                    gear_sum += n * m;
                    gear_map.remove(&s);
                } else {
                    gear_map.insert(s,number);
                }
                break
            } 
            current_x += 1;
            
        }

    }

    println!("{sum}");
    println!("Gear sum: {gear_sum}");

}
