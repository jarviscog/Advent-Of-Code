use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

fn main() {

    let file_path = "input.txt";
    //let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect(file_path);

    let mut positions: Vec<Point> = Vec::new();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for (j, line) in contents.lines().enumerate() {
        grid.push(line.chars().collect());
        if let Some(i) = line.find("S") {
            positions.push(Point::new(i, j));
        }
    }

    for i in 1..65 {

        let mut next_iteration_positions: vec<point> = vec::new();
        for p in positions.clone() {

            if p.x > 0 && grid[p.y][p.x - 1] != '#' {
                //println!("left");
                next_iteration_positions.push(point::new(p.x - 1, p.y))
            }
            if p.y > 0 && grid[p.y - 1][p.x] != '#' {
                //println!("up");
                next_iteration_positions.push(point::new(p.x, p.y - 1))
            }
            if p.x < grid[0].len() - 1 && grid[p.y][p.x + 1] != '#' {
                //println!("right");
                next_iteration_positions.push(point::new(p.x + 1, p.y))
            }
            if p.y < grid.len() - 1&& grid[p.y + 1][p.x] != '#' {
                //println!("down");
                next_iteration_positions.push(point::new(p.x, p.y + 1))
            }

        }
        next_iteration_positions.sort();
        next_iteration_positions.dedup();
        println!("steps: {} number of positions: {}", i, next_iteration_positions.len());
        positions = next_iteration_positions;
    }

    

    
}
