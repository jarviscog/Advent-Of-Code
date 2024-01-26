use core::panic;
use std::fs;
use std::time::Instant;
use petgraph::data::Build;
use petgraph::graph::{Graph};
use petgraph::algo::dijkstra;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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


fn is_junction(point: Point, grid: &Vec<Vec<bool>>) -> bool {

    let mut exit_count: u32 = 0;

    if point.x > 0 {
        let x = point.x - 1;
        let y = point.y;
        let c = grid[y][x];
        if c == true {
            exit_count += 1;
        }
    }

    if point.y > 0 {
        let x = point.x;
        let y = point.y - 1;
        let c = grid[y][x];
        if c == true {
            exit_count += 1;
        }
    }

    if point.x < grid[0].len() - 1 {
        let x = point.x + 1;
        let y = point.y;
        let c = grid[y][x];
        if c == true {
            exit_count += 1;
        }
    }

    if point.y < grid.len() - 1 {
        let x = point.x;
        let y = point.y + 1;
        let c = grid[y][x];
        if c == true {
            exit_count += 1;
        }
    }

    if exit_count > 1 {
        return true;
    } else {
        return false;
    }

}

fn next_step(point: Point, grid: &Vec<Vec<bool>>) -> Option<Point> {

    let current = grid[point.y][point.x];
    //println!("{} {}", point.x, point.y);
    if current == false {
        panic!("Started function on invalid space");
    }

    if point.x > 0 {
        let x = point.x - 1;
        let y = point.y;
        let c = grid[y][x];
        if c == true {
            return Some(Point::new(x,y));
        }
    }

    if point.y > 0 {
        let x = point.x;
        let y = point.y - 1;
        let c = grid[y][x];
        if c == true {
            return Some(Point::new(x,y));
        }
    }

    if point.x < grid[0].len() - 1 {
        let x = point.x + 1;
        let y = point.y;
        let c = grid[y][x];
        if c == true {
            return Some(Point::new(x,y));
        }
    }

    if point.y < grid.len() - 1 {
        let x = point.x;
        let y = point.y + 1;
        let c = grid[y][x];
        if c == true {
            return Some(Point::new(x,y));
        }
    }

    panic!("Could not find next step");

}

fn get_next_junction(start: Point, grid: Vec<Vec<bool>>) -> Option<(Point, u32)> {
    let mut dist = 0;
    while let Some(p) = next_step(start, &grid) {
        if is_junction(p, &grid) {
            return Some((p, dist));
        }
        dist += 1;
    }
    None
}

fn get_connected_nodes(point: Point, grid: &mut Vec<Vec<bool>>) -> Vec<(Point, u32)> {

    let current = grid[point.y][point.x];
    grid[point.y][point.x] = false;
    //println!("{} {}", point.x, point.y);
    if current == false {
        panic!("Started function on invalid space");
    }

    let mut connected_nodes: Vec<(Point, u32)> = Vec::new();

    if point.x > 0 {
        let x = point.x - 1;
        let y = point.y;
        let c = grid[y][x];
        if c == true {
            if let Some(n) = get_next_junction(Point::new(x,y), grid.clone()) {
                connected_nodes.push(n);
            }
        }
    }

    if point.y > 0 {
        let x = point.x;
        let y = point.y - 1;
        let c = grid[y][x];
        if c == true {
            if let Some(n) = get_next_junction(Point::new(x,y), grid.clone()) {
                connected_nodes.push(n);
            }
        }
    }

    if point.x < grid[0].len() - 1 {
        let x = point.x + 1;
        let y = point.y;
        let c = grid[y][x];
        if c == true {
            if let Some(n) = get_next_junction(Point::new(x,y), grid.clone()) {
                connected_nodes.push(n);
            }
        }
    }

    if point.y < grid.len() - 1 {
        let x = point.x;
        let y = point.y + 1;
        let c = grid[y][x];
        if c == true {
            if let Some(n) = get_next_junction(Point::new(x,y), grid.clone()) {
                connected_nodes.push(n);
            }
        }
    }

    connected_nodes
    
}

fn main() {

    let now = Instant::now();
    //let file_path = "input.txt";
    let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut grid: Vec<Vec<bool>> = Vec::new();
    for line in contents.lines() {
        let mut reduced_line: Vec<char> = line.chars().collect();
        reduced_line.remove(0);
        reduced_line.pop();
        grid.push(reduced_line.iter().map(|c| 
                                               match c {
                                                    '.' => true,
                                                    '<' => true,
                                                    '>' => true,
                                                    '^' => true,
                                                    'v' => true,
                                                    '#' => false,
                                                    _ => panic!("Unexpected symbol")
                                               }
                                              ).collect());
    }

    
    // Get the distance to the first junction, and the distance from the last junction to the end
    let (mut start_to_graph, mut end_to_graph) = (0, 0);
    let (starting_node, ending_node): (Point, Point);


    let mut current_point = Point::new(0,0);
    loop {
        if is_junction(current_point, &grid) {
            println!("Found junction: {} {}", current_point.x, current_point.y);
            starting_node = current_point;
            break;
        }
        let next_point = next_step(current_point, &grid);
        grid[current_point.y][current_point.x] = false;
        current_point = next_point.unwrap();
        start_to_graph += 1;
    }
    start_to_graph -= 1;

    let mut current_point = Point::new(grid[0].len() - 1, grid.len() - 1);
    loop {
        if is_junction(current_point, &grid) {
            println!("Found junction: {} {}", current_point.x, current_point.y);
            ending_node = current_point;
            break;
        }
        let next_point = next_step(current_point, &grid);
        grid[current_point.y][current_point.x] = false;
        current_point = next_point.unwrap();
        end_to_graph += 1;
    }
    end_to_graph -= 1;

    let mut junctions_to_calculate: Vec<Point> = Vec::new();
    let mut junctions_visited: Vec<Point> = Vec::new();
    junctions_to_calculate.push(starting_node);
    junctions_visited.push(starting_node);

    // Make a graph of the junctions
    let mut graph: Graph::<Point, u32> = Graph::new();

    let starting_node_index = graph.add_node(starting_node);
    let ending_node_index = graph.add_node(ending_node);

    while junctions_to_calculate.len() > 0 {
        let current_junction = junctions_to_calculate.pop().unwrap();
        junctions_visited.push(current_junction);
        let current_junction_index = graph.add_node(current_junction);
        for (node, dist) in get_connected_nodes(current_junction, &mut grid.clone()) {
            
            if !junctions_visited.contains(&node) {
                junctions_to_calculate.push(node);
                let node_index = graph.add_node(node);
                graph.add_edge(current_junction_index, node_index, dist);
            }

        graph.add_node(current_junction);
        }

        junctions_to_calculate.sort();
        junctions_to_calculate.dedup();
    }


    let dist_in_graph = dijkstra(&graph, starting_node_index, Some(ending_node_index), |_| 1);
    let dist_in_graph = 0;

    let total = start_to_graph + dist_in_graph + end_to_graph; 

    println!("Start to graph: {}", start_to_graph);
    println!("End to graph: {}", end_to_graph);
    println!("Dist in graph: {}", dist_in_graph);
    println!("Total: {}", total);
    
}
