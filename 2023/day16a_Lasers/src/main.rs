use core::panic;
use std::fs;


struct Operation {
    x: i32,
    y: i32,
    direction: Direction
}

impl Operation {
    fn new(x: i32, y: i32, direction: Direction) -> Operation {
        Operation {
            x,
            y,
            direction
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Up => {'U'}
            Direction::Right => {'R'}
            Direction::Down => {'D'}
            Direction::Left => {'L'}
        }
    
    }
}

fn main() {

    let file_path = "input.txt";
    //let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("File path");


    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut output_grid: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        grid.push(line.chars().collect());
        output_grid.push(line.chars().collect());
    }

    let mut list_of_operations: Vec<Operation> = Vec::new();
    list_of_operations.push(Operation::new(0,0,Direction::Right));

    while list_of_operations.len() > 0 {
        let current_op= list_of_operations.remove(0);
        // Remove out of bounds commands
        if current_op.x < 0 || current_op.y < 0 || current_op.x >= grid[0].len() as i32 || current_op.y >= grid.len() as i32 {
            continue;
        }
        // Remove elements that have already been calculated
        if output_grid[current_op.y as usize][current_op.x as usize] == current_op.direction.to_char() {
            continue;
        }

        let current_symbol = grid[current_op.y as usize][current_op.x as usize];
        output_grid[current_op.y as usize][current_op.x as usize] = current_op.direction.to_char();

        match current_symbol {
            '.' => {
                let next_x: i32; 
                let next_y: i32;
                match current_op.direction {
                    Direction::Left => { 
                        next_x = current_op.x - 1;
                        next_y = current_op.y;
                    }
                    Direction::Right => { 
                        next_x = current_op.x + 1;
                        next_y = current_op.y;
                    }
                    Direction::Up => { 
                        next_x = current_op.x;
                        next_y = current_op.y - 1;
                    }
                    Direction::Down => { 
                        next_x = current_op.x;
                        next_y = current_op.y + 1;
                    }
                }
                list_of_operations.push(Operation::new(next_x, next_y, current_op.direction))
            }
            '-' => {
                match current_op.direction {
                    Direction::Left => {
                        list_of_operations.push(Operation::new(current_op.x - 1, current_op.y, current_op.direction))
                    }
                    Direction::Right => {
                        list_of_operations.push(Operation::new(current_op.x + 1, current_op.y, current_op.direction))
                    }
                    Direction::Up | Direction::Down => {
                        list_of_operations.push(Operation::new(current_op.x - 1, current_op.y, Direction::Left));
                        list_of_operations.push(Operation::new(current_op.x + 1, current_op.y, Direction::Right));
                    }
                }
            }
            '|' => {
                match current_op.direction {
                    Direction::Up => {
                        list_of_operations.push(Operation::new(current_op.x, current_op.y - 1, current_op.direction))
                    }
                    Direction::Down => {
                        list_of_operations.push(Operation::new(current_op.x, current_op.y + 1, current_op.direction))
                    }
                    Direction::Left | Direction::Right => {
                        list_of_operations.push(Operation::new(current_op.x, current_op.y - 1, Direction::Up));
                        list_of_operations.push(Operation::new(current_op.x, current_op.y + 1, Direction::Down));
                    }
                }
            }
            '\\' => {
                let next_x: i32; 
                let next_y: i32;
                let next_direction: Direction;
                match current_op.direction {
                    Direction::Left => { 
                        next_x = current_op.x;
                        next_y = current_op.y - 1;
                        next_direction = Direction::Up;
                    }
                    Direction::Right => { 
                        next_x = current_op.x;
                        next_y = current_op.y + 1;
                        next_direction = Direction::Down;
                    }
                    Direction::Up => { 
                        next_x = current_op.x - 1;
                        next_y = current_op.y;
                        next_direction = Direction::Left;
                    }
                    Direction::Down => { 
                        next_x = current_op.x + 1;
                        next_y = current_op.y;
                        next_direction = Direction::Right;
                    }
                }
                list_of_operations.push(Operation::new(next_x, next_y, next_direction));
            }
            '/' => {
                let next_x: i32; 
                let next_y: i32;
                let next_direction: Direction;
                match current_op.direction {
                    Direction::Right => { 
                        next_x = current_op.x;
                        next_y = current_op.y - 1;
                        next_direction = Direction::Up;
                    }
                    Direction::Left => { 
                        next_x = current_op.x;
                        next_y = current_op.y + 1;
                        next_direction = Direction::Down;
                    }
                    Direction::Up => { 
                        next_x = current_op.x + 1;
                        next_y = current_op.y;
                        next_direction = Direction::Right;
                    }
                    Direction::Down => { 
                        next_x = current_op.x - 1;
                        next_y = current_op.y;
                        next_direction = Direction::Left;
                    }
                }
                list_of_operations.push(Operation::new(next_x, next_y, next_direction));
            }
            _ => {panic!("Unknown symbol: [{current_symbol}]")}
        }

    }

    let mut power = 0;
    for line in &output_grid {
        for c in line {
            match c {
                'L' | 'R' | 'U' | 'D' => { power += 1; print!("#"); }
                _ => { print!("."); }
            }
        }
        println!();
    }
    println!("Power: {}", power);


}
