use core::time;
use std::{fs, thread::sleep};
use std::time::Instant;


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
}

impl Node {

    fn new(location: Point, symbol: char, from: Option<Point>) -> Node {
        Node {
            location,
            symbol,
            from,
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

    let now = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    //println!("Size: {} {}", contents.lines().count(), contents.lines().nth(0).unwrap().len());
    let mut grid: Vec<Vec<char>> = Vec::new(); 

    let mut starting_point: Option<Point> = None;
    for (y, line) in contents.lines().enumerate() {
        if let Some(x) = line.find('S') {
            starting_point = Some(Point::new(x as u32,y as u32));
        }
        grid.push(line.chars().collect())
    }

    if let None = starting_point { panic!("No starting point") }

    let mut intermediate_grid = grid.clone();
    let mut current_node: Node = Node::new(starting_point.unwrap(), 'S', None);
    let mut next_symbol = 'A';

    // Mark all path nodes
    while next_symbol != 'S' {
        let next_location = current_node.next_point();
        next_symbol = grid[next_location.y as usize][next_location.x as usize].clone();
        let symbol_for_intermediate = 
            match next_symbol {
                'L' => {'l'}
                'F' => {'f'}
                '-' => {'_'}
                '|' => {'i'}
                '7' => {'T'}
                'J' => {'j'}
                'S' => {'J'}
                _ => panic!("Unexpected symbol {next_symbol}")
            };
        intermediate_grid[next_location.y as usize][next_location.x as usize] = symbol_for_intermediate;

        current_node = Node::new(next_location, next_symbol, Some(current_node.location));

        //sleep(time::Duration::from_millis(10));
    }

    // Clear all junk pipes
    for (y, line) in intermediate_grid.clone().iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            match char {
                'j' | '_' | 'f' | 'T' | 'i' | 'S' | 'l' => {
                    intermediate_grid[y][x] = match char {
                        '_' => '-',
                        'i' => '|',
                        'j' => 'J',
                        'l' => 'L',
                        'f' => 'F',
                        'T' => '7',
                        'S' => 'S',
                        _ => panic!("Unexpected symbol"),
                    };
                }
                _ => { intermediate_grid[y][x] = ' ' }
            }
        }
    }
    //println!("Remove Junk Pipe: ");
    //print_grid(&intermediate_grid);

    let mut inside_count = 0;
    // Mark all inside pieces horozontally
    for (y, line) in intermediate_grid.clone().iter().enumerate() {
        let mut in_pipe = false;
        let mut came_from_top = false;
        for (x, char) in line.iter().enumerate() {

            match char {
                '|' | 'S' => { in_pipe = !in_pipe; },
                'L' => {
                    came_from_top = true;
                }
                'F' => {
                    came_from_top = false;
                }
                '7' => { if came_from_top { in_pipe = !in_pipe } }
                'J' => { if !came_from_top { in_pipe = !in_pipe } }
                '-' => { },
                ' ' => {
                    if in_pipe { 
                        intermediate_grid[y][x] = 'A';
                    }
                }
                _ => panic!("Unexpected symbol [{char}]")
            }
        
        }
    }
    //println!("Find inside segments A");
    //print_grid(&intermediate_grid);

    for x in 0..intermediate_grid[0].len() {
        let mut in_pipe = false;
        let mut came_from_left = false;
        for y in 0..intermediate_grid.len() {
            let char = intermediate_grid[y][x];

            match char {
                '-' | 'S' => { in_pipe = !in_pipe; },
                'F' => {
                    came_from_left = false;
                }
                '7' => {
                    came_from_left = true;
                }
                'L' => {
                    if came_from_left { in_pipe = !in_pipe; }
                }
                'J' => {
                    if !came_from_left { in_pipe = !in_pipe; }
                }
                '|' => { },
                ' ' => { 
                    if in_pipe {
                    
                        intermediate_grid[y][x] = 'B';
                    }
                },
                'A' => {
                    if in_pipe { 
                        inside_count += 1;
                        intermediate_grid[y][x] = 'I';
                    } else { 
                        intermediate_grid[y][x] = ' ';
                    }
                }
                _ => panic!("Unexpected symbol [{char}]")
            }
        
        }
    }

    //println!("Find inside segments B");
    //print_grid(&intermediate_grid);

    let e = now.elapsed();
    println!("Time: {:.2?}", e);
    println!("Inside count: {}", inside_count);


}
