use std::{fs, usize};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y
        }
    }
}

fn count_positions(point: Point, grid: &Vec<Vec<char>>, steps: usize) -> usize {
    if steps == 0 {
        return 0;
    }
    if steps == 1 {
        return 1;
    }

    println!("Remaining steps: {}", steps);

    let mut positions: Vec<Point> = Vec::new();
    positions.push(point);

    for i in 0..steps - 1{

        let mut next_iteration_positions: Vec<Point> = Vec::new();
        for p in positions.clone() {

            if p.x > 0 && grid[p.y][p.x - 1] != '#' {
                //println!("left");
                next_iteration_positions.push(Point::new(p.x - 1, p.y))
            }
            if p.y > 0 && grid[p.y - 1][p.x] != '#' {
                //println!("up");
                next_iteration_positions.push(Point::new(p.x, p.y - 1))
            }
            if p.x < grid[0].len() - 1 && grid[p.y][p.x + 1] != '#' {
                //println!("right");
                next_iteration_positions.push(Point::new(p.x + 1, p.y))
            }
            if p.y < grid.len() - 1&& grid[p.y + 1][p.x] != '#' {
                //println!("down");
                next_iteration_positions.push(Point::new(p.x, p.y + 1))
            }

        }
        next_iteration_positions.sort();
        next_iteration_positions.dedup();
        positions = next_iteration_positions;
    }
    println!("steps: {} number of positions: {}", steps, positions.len());
    return positions.len(); 


}

fn main() {

    //let file_path = "input.txt";
    let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect(file_path);

    let mut positions: Vec<Point> = Vec::new();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut number_of_gardens_in_subplot = 0;
    for (j, line) in contents.lines().enumerate() {
        grid.push(line.chars().collect());
        number_of_gardens_in_subplot += line.chars().filter(|c| c.clone() != '#').count();
        if let Some(i) = line.find("S") {
            positions.push(Point::new(i, j));
        }
    }

    let num_of_steps = 27;


    // There is a diamond that shows up. Inside shapes are always covered
    let dist_to_side = (grid[0].len() - 1) / 2;
    let mut num_subplots_to_edge = (num_of_steps - dist_to_side ) / grid[0].len();
    let remainder = (num_of_steps - dist_to_side ) % grid[0].len();
    if remainder != 0 {
        num_subplots_to_edge += 1;
    } 

    let num_of_edge_subplots = num_subplots_to_edge * 4 + (num_subplots_to_edge - 1) * 4;
    let num_of_internal_subplots = (((num_subplots_to_edge * 2) + 1).pow(2) + 1)/2 - num_of_edge_subplots; // Area of a 
                                                                                                           
    let mut total = 0;

    total += num_of_internal_subplots * number_of_gardens_in_subplot;

    let starting_x = grid[0].len()/2 as usize;
    let starting_y = grid.len()/2;
    // Do the four edges
    total += count_positions(Point::new(starting_x, grid.len() - 1), &grid, remainder); // Top
    total += count_positions(Point::new(starting_x, 0), &grid, remainder); // Bottom
    total += count_positions(Point::new(grid[0].len() - 1, starting_y), &grid, remainder); // Right
    total += count_positions(Point::new(0, starting_y), &grid, remainder); // Left
    
    let steps_to_corner = 0;

    // Bottom right
    for i in 0..num_subplots_to_edge {
        //total += count_positions(Point::new(0, 0), &grid, steps_to_corner);
    }
    
    
    println!();
    println!("Number of steps: {}", num_of_steps);
    println!("Width/Height of a plot {} {}", grid[0].len(), grid.len());
    println!("Number of subplots to the right of center: {:.2}", num_subplots_to_edge);
    println!("Number of internal subplots: {}", num_of_internal_subplots);
    println!("Number of edge subplots: {}", num_of_edge_subplots);
    println!("Number of gardens in subplot: {}", number_of_gardens_in_subplot);
    println!("Total: {}", total);
    // Number of subplots to the right that are able to be completely traversed

    //println!("Total: {}", total);


    

    
}
