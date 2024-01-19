use std::{fs, usize};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    direction: Direction,
    amount_travelled: u32,
    visited: bool,
    cost: u32,
    from_x: usize,
    from_y: usize,
}

impl Node {
    fn new(x: usize,
           y: usize,
           direction: Direction,
           amount_travelled: u32,
           visited: bool,
           cost: u32,
           from_x: usize,
           from_y: usize) -> Node {
        Node {
            x,
            y,
            direction,
            amount_travelled,
            visited,
            cost,
            from_x,
            from_y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    None,
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::None => '.',
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::East => 'E',
            Direction::West => 'W',
        }
    }
}

fn main() {

    let file_path = "input.txt";
    //let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("File path");

    let mut grid: Vec<Vec<Node>> = Vec::new();
    let mut cost_grid: Vec<Vec<u16>> = Vec::new();

    for (j, line) in contents.lines().enumerate() {
        grid.push(Vec::new());
        cost_grid.push(line.chars().map(|c| c.to_digit(10).unwrap() as u16).collect());

        for (i, char) in line.chars().enumerate() {

            grid[j].push(
                Node::new(
                    i,
                    j,
                    Direction::None,
                    0,
                    false,
                    999,
                    0,
                    0,
                    )
                )
        }
    }
    let width = grid[0].len();
    let height = grid.len();
    let mut paths: Vec<Node> = Vec::new();
    grid[0][0].cost = 0;
    paths.push(grid[0][0]);
    

    let mut current_node = grid[0][0].clone();

    while current_node.x != width - 1 || current_node.y != height - 1 {

        paths.sort_by_key(|n| n.cost.clone());
        paths.dedup();

        current_node = paths.remove(0).clone();
        if current_node.visited == true {
            continue;
        }
        println!("Current node: ({}, {})", current_node.x, current_node.y);
        //println!("Cost to get here: {} Direction: {}", current_node.cost, current_node.direction.to_char());

        //println!("{} > {}", grid[current_node.y][current_node.x + 1].cost, 
                 //current_node.cost + cost_grid[current_node.y as usize][current_node.x as usize + 1] as u32 );
            
        grid[current_node.y][current_node.x].visited = true;
        grid[current_node.y][current_node.x].direction = current_node.direction;
        grid[current_node.y][current_node.x].amount_travelled = current_node.amount_travelled;
        grid[current_node.y][current_node.x].cost = current_node.cost;
        grid[current_node.y][current_node.x].from_x = current_node.from_x;
        grid[current_node.y][current_node.x].from_y = current_node.from_y;

        
        let new_x = current_node.x - 1;
        let new_y = current_node.y;
        // Add the adjacent nodes to the distances
        if current_node.x > 0 
            && current_node.direction != Direction::East
                && (current_node.direction != Direction::West || current_node.amount_travelled < 3) 
                    && grid[new_y][new_x].visited == false 
                    && grid[new_y][new_x].cost > current_node.cost + cost_grid[current_node.y as usize][current_node.x as usize - 1] as u32 {

                        let new_amount_travelled: u32;
                        if current_node.direction == Direction::West {
                            new_amount_travelled = current_node.amount_travelled + 1;
                        } else {
                            new_amount_travelled = 1;
                        }

                        let new_cost = current_node.cost + cost_grid[current_node.y as usize][current_node.x as usize - 1] as u32;

                        //println!("Push west");
                        paths.push(
                            Node::new(
                                new_x,
                                new_y,
                                Direction::West,
                                new_amount_travelled,
                                false,
                                new_cost,
                                current_node.x,
                                current_node.y
                                      ))

                    }



        let new_x = current_node.x;
        let new_y = current_node.y - 1;
        if current_node.y > 0
            && current_node.direction != Direction::South
                && (current_node.direction != Direction::North || current_node.amount_travelled < 3) 
                    && grid[new_y][new_x].visited == false
                    && grid[new_y][new_x].cost > current_node.cost + cost_grid[current_node.y as usize - 1][current_node.x as usize] as u32 {

                        let new_amount_travelled: u32;
                        if current_node.direction == Direction::North {
                            new_amount_travelled = current_node.amount_travelled + 1;
                        } else {
                            new_amount_travelled = 1;
                        }

                        let new_cost = current_node.cost + cost_grid[current_node.y as usize - 1][current_node.x as usize] as u32;
                        //println!("Push North");
                        paths.push(
                            Node::new(
                                new_x,
                                new_y,
                                Direction::North,
                                new_amount_travelled,
                                false,
                                new_cost,
                                current_node.x,
                                current_node.y
                                      ))

                    }

        let new_x = current_node.x + 1;
        let new_y = current_node.y;
        if current_node.x < width - 1 
            && current_node.direction != Direction::West
                && (current_node.direction != Direction::East || current_node.amount_travelled < 3) 
                    && grid[new_y][new_x].visited == false 
                    && grid[new_y][new_x].cost > current_node.cost + cost_grid[current_node.y as usize][current_node.x as usize + 1] as u32 { 

                        let new_amount_travelled: u32;
                        if current_node.direction == Direction::East {
                            new_amount_travelled = current_node.amount_travelled + 1;
                        } else {
                            new_amount_travelled = 1;
                        }

                        let new_cost = grid[current_node.y][current_node.x].cost + cost_grid[current_node.y as usize][current_node.x as usize + 1] as u32;
                        //println!("Push east");
                        paths.push(
                            Node::new(
                                new_x,
                                new_y,
                                Direction::East,
                                new_amount_travelled,
                                false,
                                new_cost,
                                current_node.x,
                                current_node.y
                                      ))

                    }

        let new_x = current_node.x;
        let new_y = current_node.y + 1;
        if current_node.y < height - 1 
            && current_node.direction != Direction::North
                && (current_node.direction != Direction::South || current_node.amount_travelled < 3) 
                    && grid[new_y][new_x].visited == false 
                    && grid[new_y][new_x].cost > current_node.cost + cost_grid[current_node.y as usize + 1][current_node.x as usize] as u32 {

                        let new_amount_travelled: u32;
                        if current_node.direction == Direction::South {
                            new_amount_travelled = current_node.amount_travelled + 1;
                        } else {
                            new_amount_travelled = 1;
                        }

                        let new_cost = current_node.cost + cost_grid[current_node.y as usize + 1][current_node.x as usize] as u32;
                        //println!("Push South");
                        paths.push(
                            Node::new(
                                new_x,
                                new_y,
                                Direction::South,
                                new_amount_travelled,
                                false,
                                new_cost,
                                current_node.x,
                                current_node.y
                                ))

                    }

    }

    println!("Done!");
    println!("Current node ({}, {}) : {}!", current_node.x, current_node.y, current_node.cost);
    for line in grid {

        for node in line{
            print!("{:3} ", node.cost)
        }
        println!()
    }


}
