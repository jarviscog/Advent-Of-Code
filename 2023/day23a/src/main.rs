use core::panic;
use std::fs;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y
        }
    }
}

static LEFT_OK: [u8; 2] = [b'.', b'<'];
static RIGHT_OK: [u8; 2] = [b'.', b'>'];
static UP_OK: [u8; 2] = [b'.', b'^'];
static DOWN_OK: [u8; 2] = [b'.', b'v'];

fn get_longest_path(location: Point, grid: &mut Vec<Vec<char>>) -> (u32, Vec<Vec<char>>) {

    
    let current = grid[location.y][location.x];
    //println!("{} {}", location.x, location.y);
    if current == '#' || current == 'O' {
        panic!("Started function on invalid space");
    }
    grid[location.y][location.x] = 'O';
    let mut longest = 0;
    let mut winning_grid: Vec<Vec<char>> = grid.clone();


    if location.x > 0 {
        let x = location.x - 1;
        let y = location.y;
        let c = grid[y][x];
        if LEFT_OK.contains(&(c as u8)) {
            //println!("Tried left");
            let (length, ret_grid) = get_longest_path(Point::new(x, y), &mut grid.clone());
            if length > longest {
                winning_grid = ret_grid;
                longest = length + 1;
            }
        }
    }

    if location.y > 0 {
        let x = location.x;
        let y = location.y - 1;
        let c = grid[y][x];
        if UP_OK.contains(&(c as u8)) {
            //println!("Tried up");
            let (length, ret_grid) = get_longest_path(Point::new(x, y), &mut grid.clone());
            if length > longest {
                winning_grid = ret_grid;
                longest = length + 1;
            }
        }
    }

    if location.x < grid[0].len() - 1 {
        let x = location.x + 1;
        let y = location.y;
        let c = grid[y][x];
        if RIGHT_OK.contains(&(c as u8)) {
            //println!("Tried Right");
            let (length, ret_grid) = get_longest_path(Point::new(x, y), &mut grid.clone());
            if length > longest {
                winning_grid = ret_grid;
                longest = length + 1;
            }
        }
    }

    if location.y < grid.len() - 1 {
        let x = location.x;
        let y = location.y + 1;
        let c = grid[y][x];
        if DOWN_OK.contains(&(c as u8)) {
            //println!("Tried down");
            let (length, ret_grid) = get_longest_path(Point::new(x, y), &mut grid.clone());
            if length > longest {
                winning_grid = ret_grid;
                longest = length + 1;
            }
        }
    }

    if longest == 0 {
        longest = 1;
    }

    (longest, winning_grid)

}

fn main() {

    let file_path = "input.txt";
    //let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        grid.push(line.chars().collect());
    }

    let (total, winning_grid) = get_longest_path(Point::new(1, 0), &mut grid.clone());

    println!("Winning path: ");
    for row in winning_grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!("Total: {}", total);
    
}
