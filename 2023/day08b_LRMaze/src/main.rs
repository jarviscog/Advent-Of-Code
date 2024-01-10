use std::fs;
use std::collections::HashMap;
use num::integer::lcm;
use std::time::Instant;

struct Node {
    left: String,
    right: String
}
impl Node {

    fn new(left: String, right: String) -> Node {
        Node {
            left,
            right,
        }
    }

}

fn get_number_of_turns(map: &HashMap<String, Node>, lr_values: &str, starting_node: &str) -> usize {
    let mut navigation_path: String = String::new();
    let mut current_pos = 0;
    let mut current_node: String = String::from(starting_node);
    while current_node.chars().last().unwrap() != 'Z' {
            //println!("current node: [{}]", current_node);
            if let Some(n) = map.get(&current_node) {
                //println!("{} {} {}", current_node, n.left, n.right);
            }
        match lr_values.chars().nth(current_pos).unwrap() {
            'L' => {
                //println!("left from [{}]", current_node);
                //println!("{}", map.get(&current_node).unwrap().left);
                current_node = map.get(&current_node).unwrap().left.clone();
                navigation_path.push('L');
            }
            'R' => {
                //println!("right from [{}]", current_node);
                //println!("{}", map.get(&current_node).unwrap().right);
                current_node = map.get(&current_node).unwrap().right.clone();
                navigation_path.push('R');
            }
            _ => panic!("Unexpected direction")
        }
        current_pos += 1;
        if current_pos == lr_values.len() {
            current_pos = 0;
        }
    }
    navigation_path.len()
}

fn main() {

    let now = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");
    let mut lines = contents.split("\n").filter(|x| !x.is_empty());
    let lr_values: String = lines.nth(0).unwrap().to_string(); // This pops the value
    
    let mut map: HashMap<String, Node> = HashMap::new();
    let mut starting_nodes: Vec<&str> = Vec::new();

    for line in lines {
        let (mut key, lr_nodes) = line.split_once("=").unwrap();
        let (left_raw, right_raw) = lr_nodes.split_once(",").unwrap();
        // Some cleaning
        key = key.trim();
        let left: String = left_raw.chars().filter( |n| !n.is_ascii_punctuation() && !n.is_whitespace()).collect();
        let right: String = right_raw.chars().filter( |n| !n.is_ascii_punctuation() && !n.is_whitespace()).collect();
        //println!("Line: {} L:[{}] R:[{}]", line, left, right);
        map.insert(key.to_string(), Node::new(left, right));

        if key.chars().last().unwrap() == 'A' {
            starting_nodes.push(key);
        }
    }

    let mut cycle_times: Vec<u64> = Vec::new();
    // For each node, find the cycle time, e.g. node AAZ lands on a XXZ node every 2304 times
    while let Some(node) = starting_nodes.pop() {
        //println!("Node: {} Steps to go around: {}", node, get_number_of_turns(&map, &lr_values, &node) as u64);
        cycle_times.push(get_number_of_turns(&map, &lr_values, &node) as u64);

    }

    let mut ans = 1;
    while let Some(n) = cycle_times.pop() {
        ans = lcm(ans, n);
    }


    println!("{:.2?}", now.elapsed());
    println!("LCM: {}", ans);
    
}
