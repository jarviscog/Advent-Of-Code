use core::time;
use std::{fs, thread::sleep};

struct Point {
    x: u32,
    y: u32
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

struct Node {
    location: Point,
    symbol: char,
    from: Option<Point>,
    to: Option<Point>
}

impl Node {

    fn new(location: Point, symbol: char, from: Option<Point>, to: Option<Point>) -> Node {
        Node {
            location,
            symbol,
            from,
            to
        }
    }

    fn next_point(&self) -> Point {

        let x_diff: i32;
        let y_diff: i32;
        // Get the diff between this point and the last point
        if let Some(p) = &self.from {
            x_diff = p.x as i32 - self.location.x as i32;
            y_diff = p.y as i32 - self.location.y as i32;

        } else if self.symbol == 'S' {
            x_diff = -1;
            y_diff = 0;
        } else {
            panic!("No segment to come from, and not at start");
        }
        match self.symbol {
            'S' => { Point::new((self.location.x as i32 + x_diff) as u32, (self.location.y as i32 + y_diff) as u32) }
            '-' => { Point::new((self.location.x as i32 - x_diff) as u32, self.location.y )}
            '|' => { Point::new(self.location.x, (self.location.y as i32 - y_diff) as u32) }
            'F' => { Point::new((self.location.x as i32 + y_diff) as u32, (self.location.y as i32 + x_diff) as u32) }
            '7' => { Point::new((self.location.x as i32 - y_diff) as u32, (self.location.y as i32 - x_diff) as u32) }
            'J' => { Point::new((self.location.x as i32 + y_diff) as u32, (self.location.y as i32 + x_diff) as u32) }
            'L' => { Point::new((self.location.x as i32 - y_diff) as u32, (self.location.y as i32 - x_diff) as u32) }
            _ => { panic!("Unexpected symbol: [{}]", self.symbol)}
        }
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for (i, line) in grid.iter().enumerate() {
        print!("{:<5}", i);
        for char in line {
            print!("{}", char);
        }
        println!("");
    }
}

fn main() {

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    println!("Size: {} {}", contents.lines().count(), contents.lines().nth(0).unwrap().len());

    let mut grid: Vec<Vec<char>> = Vec::new(); 

    let mut starting_point: Option<Point> = None;
    for (y, line) in contents.lines().enumerate() {
        if let Some(x) = line.find('S') {
            starting_point = Some(Point::new(x as u32,y as u32));
        }
        grid.push(line.chars().collect())
    }
    if let None = starting_point { panic!("No starting point") }

    let mut printed_grid = grid.clone();
    let mut current_node: Node = Node::new(starting_point.unwrap(), 'S', None, None);
    let mut next_symbol = 'A';

    let mut moves = 0;
    // Mark all path nodes
    while next_symbol != 'S' {
        print_grid(&printed_grid);
        let next_location = current_node.next_point();

        next_symbol = grid[next_location.y as usize][next_location.x as usize].clone();
        printed_grid[next_location.y as usize][next_location.x as usize] = '*';

        current_node = Node::new(next_location, next_symbol, Some(current_node.location), None);
        moves += 1;
        //sleep(time::Duration::from_millis(10));
    }

    println!("{}", moves);
    println!("{}", moves/2);

}
