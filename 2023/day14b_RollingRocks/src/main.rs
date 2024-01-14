use std::{fs, u128};
use std::collections::HashMap;

fn roll_north(grid: &mut Vec<Vec<char>>, width: usize, height: usize) {
    for x in 0..width {
        for _ in 0..&height + 1 {
            for y in 0..height {
                if y < height - 1 {
                    if grid[y][x] == '.' && grid[y + 1][x] == 'O' {
                        grid[y][x] = 'O';
                        grid[y + 1][x] = '.';
                    }
                }
            }

        }
    }
}

fn roll_west(grid: &mut Vec<Vec<char>>, width: usize, height: usize) {
    for y in 0..height {
        for _ in 0..&width + 1 {
            for x in 0..width {
                if x < width - 1 {
                    if grid[y][x] == '.' && grid[y][x + 1] == 'O' {
                        grid[y][x] = 'O';
                        grid[y][x + 1] = '.';
                    }
                }
            }

        }
    }
}

fn roll_south(grid: &mut Vec<Vec<char>>, width: usize, height: usize) {
    for x in 0..width {
        for _ in 0..&height + 1 {
            for y in 0..height {
                if y < height - 1 {
                    if grid[y + 1][x] == '.' && grid[y][x] == 'O' {
                        grid[y + 1][x] = 'O';
                        grid[y][x] = '.';
                    }
                }
            }

        }
    }
}

fn roll_east(grid: &mut Vec<Vec<char>>, width: usize, height: usize) {
    for y in 0..height {
        for _ in 0..&width + 1 {
            for x in 0..width {
                if x < width - 1{
                    if grid[y][x + 1] == '.' && grid[y][x] == 'O' {
                        grid[y][x + 1] = 'O';
                        grid[y][x] = '.';
                    }
                }
            }

        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East
}


fn check_position(map: &mut HashMap<(Vec<u128>, Direction), u32>, grid: &Vec<Vec<char>>, direction: Direction, cycle_number: u32) -> u32 {
    let mut position_value: Vec<u128> = Vec::new();
    for line in grid.iter() {
        let line_value: u128 = u128::from_str_radix(&line.into_iter().map(|c|
            match c {
                'O' => '1',
                '.' | '#' => '0',
                _ => panic!("Unexpected symbol")
            }
            ).collect::<String>(), 2).unwrap();
        position_value.push(line_value);
    }
    if let Some(c) = map.get(&(position_value.clone(), direction.clone())) {
        return c.clone();
    } else {
        map.insert((position_value, direction), cycle_number);
        return 0;
    }
     
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
    println!();

    let width = grid[0].len();
    let height = grid.len();

    let mut map: HashMap<(Vec<u128>, Direction), u32> = HashMap::new();


    let total_cycles = 1000000000;
    let mut current_cycle = 0;
    while current_cycle < total_cycles {

        let cycle_pos = check_position(&mut map, &grid, Direction::North, current_cycle);
        if cycle_pos > 0 {
            println!("Found a cycle from {} to {}!", cycle_pos, current_cycle);
            
            let difference = current_cycle - cycle_pos;
            let cycles_to_jump = (total_cycles - current_cycle) / difference;
            current_cycle += cycles_to_jump * difference;

            roll_north(&mut grid, width, height);
            roll_west(&mut grid, width, height);
            roll_south(&mut grid, width, height);
            roll_east(&mut grid, width, height);
            current_cycle += 1;
            continue;
        }

        roll_north(&mut grid, width, height);

        let cycle_pos = check_position(&mut map, &grid, Direction::West, current_cycle);
        if cycle_pos > 0 {
            println!("Found a cycle from {} to {}!", cycle_pos, current_cycle);
            
            let difference = current_cycle - cycle_pos;
            let cycles_to_jump = (total_cycles - current_cycle) / difference;
            current_cycle += cycles_to_jump * difference;

            roll_west(&mut grid, width, height);
            roll_south(&mut grid, width, height);
            roll_east(&mut grid, width, height);
            current_cycle += 1;
            continue;
        }

        roll_west(&mut grid, width, height);

        let cycle_pos = check_position(&mut map, &grid, Direction::South, current_cycle);
        if cycle_pos > 0 {
            println!("Found a cycle from {} to {}!", cycle_pos, current_cycle);
            
            let difference = current_cycle - cycle_pos;
            let cycles_to_jump = (total_cycles - current_cycle) / difference;
            current_cycle += cycles_to_jump * difference;

            roll_south(&mut grid, width, height);
            roll_east(&mut grid, width, height);
            current_cycle += 1;
            continue;
        }

        roll_south(&mut grid, width, height);

        let cycle_pos = check_position(&mut map, &grid, Direction::South, current_cycle);
        if cycle_pos > 0 {
            println!("Found a cycle from {} to {}!", cycle_pos, current_cycle);
            
            let difference = current_cycle - cycle_pos;
            let cycles_to_jump = (total_cycles - current_cycle) / difference;
            current_cycle += cycles_to_jump * difference;

            roll_east(&mut grid, width, height);
            current_cycle += 1;
            continue;
        }

        roll_east(&mut grid, width, height);
        current_cycle += 1;
    }

    let mut sum = 0;
    for i in 0..height {
        sum += (height - i) * grid[i].clone().into_iter().filter(|n| *n == 'O').count();
    }
    println!("Sum: {}", sum);


}
