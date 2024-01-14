use std::fs;
use std::time::Instant;

fn roll_rocks(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {

    for x in 0..grid[0].len() {
        //println!("Column {}", x);
        for _ in 0..&grid.len() + 1 {
            for y in 0..grid.len() {
                if y < grid.len() - 1 {
                    //println!("{} vs. {}", grid[y][x], grid[y + 1][x]);
                    if grid[y][x] == '.' && grid[y + 1][x] == 'O' {
                        grid[y][x] = 'O';
                        grid[y + 1][x] = '.';
                    }
                }
            }

        }
    }
    return grid;

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

    let now = Instant::now();
    grid = roll_rocks(grid);
    println!("Time to roll: {:.2?}", now.elapsed());


    for line in &grid {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    let height = grid.len();

    let mut sum = 0;
    for i in 0..height {
        sum += (height - i) * grid[i].clone().into_iter().filter(|n| *n == 'O').count();
    }
    println!("Sum: {}", sum);


}
