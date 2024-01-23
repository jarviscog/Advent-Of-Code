use core::panic;
use std::{fs, usize, collections::HashMap};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
impl Point {
    fn new(x:i32, y:i32, z:i32) -> Point {
        Point {
            x,
            y,
            z,
        }
    }
    fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy)]
struct Brick {
    brick_number: u32,
    a: Point,
    b: Point,
}
impl Brick {
    fn new(brick_number: u32, a:Point , b:Point) -> Brick {
        Brick {
            brick_number,
            a,
            b,
        }
    }
    fn to_string(&self) -> String {
        format!("{}: {}, {}", self.brick_number, self.a.to_string(), self.b.to_string())
    }
}

fn place_brick(brick: &Brick, grid: &mut Vec<Vec<Vec<u32>>>) {
    // Places a brick in the grid
    let x_diff = brick.b.x - brick.a.x;
    let y_diff = brick.b.y - brick.a.y;
    let z_diff = brick.b.z - brick.a.z;

    if x_diff > 0 {
        for i in 0..x_diff + 1 {
            grid[brick.a.z as usize][brick.a.y as usize][brick.a.x as usize + i as usize] = brick.brick_number;
        }
    } else if y_diff > 0 {
        for i in 0..y_diff + 1 {
            grid[brick.a.z as usize][brick.a.y as usize + i as usize][brick.a.x as usize] = brick.brick_number;
        }
    } else if z_diff > 0 {
        for i in 0..z_diff + 1 {
            grid[brick.a.z as usize + i as usize][brick.a.y as usize][brick.a.x as usize] = brick.brick_number;
        }
    } else if x_diff == 0 && y_diff == 0 && z_diff == 0 {
        grid[brick.a.z as usize][brick.a.y as usize][brick.a.x as usize] = brick.brick_number;
    } else {
        panic!("The brick is not a line");
    }
    
}

fn remove_brick(brick: &Brick, grid: &mut Vec<Vec<Vec<u32>>>) {

    // Removes a brick in the grid
    let x_diff = brick.b.x - brick.a.x;
    let y_diff = brick.b.y - brick.a.y;
    let z_diff = brick.b.z - brick.a.z;

    if x_diff > 0 {
        for i in 0..x_diff + 1 {
            grid[brick.a.z as usize][brick.a.y as usize][brick.a.x as usize + i as usize] = 0;
        }
    } else if y_diff > 0 {
        for i in 0..y_diff + 1 {
            grid[brick.a.z as usize][brick.a.y as usize + i as usize][brick.a.x as usize] = 0;
        }
    } else if z_diff > 0 {
        for i in 0..z_diff + 1 {
            grid[brick.a.z as usize + i as usize][brick.a.y as usize][brick.a.x as usize] = 0;
        }
    } else if x_diff == 0 && y_diff == 0 && z_diff == 0 {
        grid[brick.a.z as usize][brick.a.y as usize][brick.a.x as usize] = 0;
    } else {
        panic!("The brick is not a line");
    }
    
}

fn dist_in_air(brick: &Brick, grid: &Vec<Vec<Vec<u32>>>) -> usize {

    let x_diff = brick.b.x - brick.a.x;
    let y_diff = brick.b.y - brick.a.y;
    let z_diff = brick.b.z - brick.a.z;

    let mut dist_below_block = 1;
    //println!("Brick: {}", brick.to_string());

    if x_diff > 0 {
        loop {
            if brick.a.z - dist_below_block == 0 {
                return dist_below_block as usize - 1;
            }
            for i in 0..x_diff + 1 {
                let below = grid[brick.a.z as usize - dist_below_block as usize][brick.a.y as usize][brick.a.x as usize + i as usize];
                //println!("Location: {} {} {}", brick.a.x + i, brick.a.y, brick.a.z - dist_below_block);
                //println!("Below: {}", below);
                if below != 0 {
                    return dist_below_block as usize - 1;
                }
            } dist_below_block += 1;
        }
        
    } else if y_diff > 0 {
        loop {
            //println!("Distance to ground: {}", dist_to_ground);
            //println!("Distance below block: {}", dist_below_block);
            if brick.a.z - dist_below_block == 0 {
                return dist_below_block as usize - 1;
            }
            for i in 0..y_diff + 1 {
                let below = grid[brick.a.z as usize - dist_below_block as usize][brick.a.y as usize + i as usize][brick.a.x as usize];
                //println!("Below: {}", below);
                if below != 0 {
                    return dist_below_block as usize - 1;
                }
            }
            dist_below_block += 1;
        }
    } else if z_diff > 0 || (x_diff == 0 && y_diff == 0 && z_diff == 0){
        loop {
            //println!("Distance to ground: {}", dist_to_ground);
            if brick.a.z - dist_below_block == 0 {
                return dist_below_block as usize - 1;
            }
            //println!("Below: {}", grid[brick.a.z as usize - dist_below_block as usize][brick.a.y as usize][brick.a.x as usize] );
            if grid[brick.a.z as usize - dist_below_block as usize][brick.a.y as usize][brick.a.x as usize] != 0 {
                return dist_below_block as usize - 1;
            }
            dist_below_block += 1;
        }
    } else {
        panic!("The brick is not a line");
    }
    
}

fn get_supported_bricks(brick: &Brick, grid: &Vec<Vec<Vec<u32>>>) -> Vec<u32> {
    // Get the bricks that are supported by this brick

    let x_diff = brick.b.x - brick.a.x;
    let y_diff = brick.b.y - brick.a.y;
    let z_diff = brick.b.z - brick.a.z;

    //println!("Brick: {}", brick.to_string());
    let mut supporting_bricks = Vec::new();

    if x_diff > 0 {
        for i in 0..x_diff + 1 {
            let above = grid[brick.a.z as usize + 1][brick.a.y as usize][brick.a.x as usize + i as usize];
            if !supporting_bricks.contains(&above) && above != 0 {
                supporting_bricks.push(above);
            }
        }
    } else if y_diff > 0 {
        for i in 0..y_diff + 1 {
            let above = grid[brick.a.z as usize + 1][brick.a.y as usize + i as usize][brick.a.x as usize];
            if !supporting_bricks.contains(&above) && above != 0 {
                supporting_bricks.push(above);
            }
        }
    } else if z_diff > 0 || (x_diff == 0 && y_diff == 0 && z_diff == 0) {
        let above = grid[brick.b.z as usize + 1][brick.b.y as usize][brick.b.x as usize];
        if !supporting_bricks.contains(&above) && above != 0 {
            supporting_bricks.push(above);
        }
    } 

    else {
        panic!("The brick is not a line");
    }

    supporting_bricks
}

fn get_supporting_bricks(brick: &Brick, grid: &Vec<Vec<Vec<u32>>>) -> Vec<u32> {
    // Get the bricks that support this brick

    let x_diff = brick.b.x - brick.a.x;
    let y_diff = brick.b.y - brick.a.y;
    let z_diff = brick.b.z - brick.a.z;

    //println!("Brick: {}", brick.to_string());
    let mut supporting_bricks = Vec::new();

    if x_diff > 0 {
        for i in 0..x_diff + 1 {
            let below = grid[brick.a.z as usize - 1][brick.a.y as usize][brick.a.x as usize + i as usize];
            if !supporting_bricks.contains(&below) && below != 0 {
                supporting_bricks.push(below);
            }
        }
    } else if y_diff > 0 {
        for i in 0..y_diff + 1 {
            let below = grid[brick.a.z as usize - 1][brick.a.y as usize + i as usize][brick.a.x as usize];
            if !supporting_bricks.contains(&below) && below != 0 {
                supporting_bricks.push(below);
            }
        }
    } else if z_diff > 0 || (x_diff == 0 && y_diff == 0 && z_diff == 0) {
        let below = grid[brick.a.z as usize - 1][brick.a.y as usize][brick.a.x as usize];
        if !supporting_bricks.contains(&below) && below != 0 {
            supporting_bricks.push(below);
        }
    } else {
        panic!("The brick is not a line");
    }

    supporting_bricks

}

fn main() {

    //let file_path = "input.txt";
    let file_path = "test_one_input.txt";
    //let file_path = "test_two_input.txt";
    //let file_path = "test_three_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut grid_width = 0;
    let mut grid_height = 0;
    let mut grid_depth = 0;
    let mut bricks: Vec<Brick> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        let (point_one_str, point_two_str) = line.split_once("~").unwrap();
        let x = point_one_str.split(",").nth(0).unwrap().parse().unwrap();
        let y = point_one_str.split(",").nth(1).unwrap().parse().unwrap();
        let z = point_one_str.split(",").nth(2).unwrap().parse().unwrap();
        let point_one = Point::new(x, y, z);
        let x = point_two_str.split(",").nth(0).unwrap().parse().unwrap();
        let y = point_two_str.split(",").nth(1).unwrap().parse().unwrap();
        let z = point_two_str.split(",").nth(2).unwrap().parse().unwrap();
        let point_two = Point::new(x, y, z);

        bricks.push(Brick::new(i as u32 + 1, point_one, point_two));

        if point_two.x > grid_width { grid_width = point_two.x }
        if point_two.y > grid_height { grid_height = point_two.y }
        if point_two.z > grid_depth { grid_depth = point_two.z }
    }
    println!("Dimensions: {} {} {}", grid_width, grid_height, grid_depth);

    //let mut grid: Vec<Vec<Vec<char>>> = Vec::new();
    let mut grid: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; grid_width as usize + 2]; grid_height as usize + 2]; grid_depth as usize + 2];

    for brick in &bricks {
        //println!("{}", brick.to_string());
        place_brick(&brick, &mut grid);
    }
    let mut still_floating_bricks = true;

    let original_bricks = bricks.clone();

    while still_floating_bricks {

        // Settle all of the bricks
        for (i, brick) in bricks.clone().iter().enumerate() {

            //println!("Current brick: {}", brick.to_string());
            let dist = dist_in_air(&brick, &grid);
            //println!("Dist in air: {}", dist);
            if dist > 0 {
                //println!("Brick needs to be dropped");
                let a = Point::new(brick.a.x, brick.a.y, brick.a.z - dist as i32);
                let b = Point::new(brick.b.x, brick.b.y, brick.b.z - dist as i32);
                let new_brick = Brick::new(brick.brick_number, a, b);
                println!("Removing {}", brick.to_string());
                remove_brick(&brick, &mut grid);
                println!("Placing {}", new_brick.to_string());
                place_brick(&new_brick, &mut grid);

                bricks[i] = new_brick;

            }
        }
        
        // Are there any floating bricks?
        still_floating_bricks = false;
        for brick in &bricks {
           if dist_in_air(brick, &grid) > 0 {
                still_floating_bricks = true;
           }
        }
        
    }

    // Make a map of the brick number to the brick for lookups
    let mut bricks_map: HashMap<u32, Brick> = HashMap::new();
    for i in 0..bricks.len() {
        bricks_map.insert(bricks[i].brick_number, bricks[i].clone());
        println!("{} -> {}", original_bricks[i].to_string(), bricks[i].to_string());
    }
    

    // Run some tests
    let mut check_map: HashMap<Point, u32> = HashMap::new();
    for brick in &bricks {
        let x_diff = brick.b.x - brick.a.x;
        let y_diff = brick.b.y - brick.a.y;
        let z_diff = brick.b.z - brick.a.z;

        if x_diff > 0 {
            for i in 0..x_diff + 1 { 
                let point = Point::new(brick.a.x + i, brick.a.y, brick.a.z);
                if let Some(n) = check_map.get(&point) {
                    println!("Conflict: {} Point: {}", n, point.to_string());
                    panic!();
                } else {
                    check_map.insert(point, brick.brick_number);
                }
            }
        } else if y_diff > 0 {
            for i in 0..y_diff + 1 { 
                let point = Point::new(brick.a.x, brick.a.y + i, brick.a.z);
                if let Some(n) = check_map.get(&point) {
                    println!("Conflict: {} Point: {}", n, point.to_string());
                    panic!();
                } else {
                    check_map.insert(point, brick.brick_number);
                }
            }
        } else if z_diff > 0 {
            for i in 0..y_diff + 1 { 
                let point = Point::new(brick.a.x, brick.a.y, brick.a.z + i);
                if let Some(n) = check_map.get(&point) {
                    println!("Conflict: {} Point: {}", n, point.to_string());
                    panic!();
                } else {
                    check_map.insert(point, brick.brick_number);
                }
            }
        } else {
            //println!("Loner: {}", brick.brick_number);
            let point = Point::new(brick.a.x, brick.a.y, brick.a.z);
            check_map.insert(point, brick.brick_number);
        }
        println!("Brick: {}", brick.to_string());
        print!("supporting: ");
        for no in get_supported_bricks(brick, &grid) {
            print!("{} ", no);
            if no == brick.brick_number {
                panic!();
            }
        }
        println!();
        print!("supported by: ");
        for no in get_supporting_bricks(brick, &grid) {
            print!("{} ", no);
            if no == brick.brick_number {
                panic!();
            }
        }
        println!();
        println!();
    }





    println!("Finding bricks to remove");
    let mut removable_bricks: Vec<u32> = Vec::new();
    let mut total = 0;
    for brick in bricks {
        println!("Brick: {}", brick.brick_number);

        let supported_bricks = get_supported_bricks(&brick, &grid);
        if supported_bricks.len() == 0 { // There are no bricks to support
            if !removable_bricks.contains(&brick.brick_number) {
                println!("Can be removed: {}", brick.brick_number);
                removable_bricks.push(brick.brick_number.clone());
                total += 1;
            }
        } else {
            let mut required = false;
            // For each brick supported, check the number of bricks supported by it
            for brick_no in supported_bricks {
                let bl = get_supporting_bricks(&bricks_map[&brick_no], &grid); 
                if bl.len() <= 1 {
                    required = true;
                }
            }
            if required == false {
                if !removable_bricks.contains(&brick.brick_number) {
                    println!("Can be removed: {}", brick.brick_number);
                    removable_bricks.push(brick.brick_number.clone());
                    total += 1;
                }
            }
        }
    }

    println!("Total: {}", total);

}
