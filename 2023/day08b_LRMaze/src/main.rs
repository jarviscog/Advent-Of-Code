use std::fs;
use std::collections::HashMap;

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
        if current_node == "FMP" {
            println!("current node: {}", current_node);
            if let Some(n) = map.get(&current_node) {
                println!("{} {}", n.left, n.right);
            }
        }
        match lr_values.chars().nth(current_pos).unwrap() {
            'L' => {
                if current_node == "FMP" {
                    println!("left from [{}]", current_node);
                    println!("{}", map.get(&current_node).unwrap().left);
                }
                current_node = map.get(&current_node).unwrap().left.clone();
                navigation_path.push('L');
            }
            'R' => {
                if current_node == "FMP" {
                    println!("right");
                }
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

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");
    let mut lines = contents.split("\n").filter(|x| !x.is_empty());
    let lr_values: String = lines.nth(0).unwrap().to_string(); // This pops the value
    lines.nth(0); // Pops a blank line
    
    let mut map: HashMap<String, Node> = HashMap::new();
    let mut starting_nodes: Vec<&str> = Vec::new();
    println!("Start");
    for line in lines {
        let (mut key, lr_nodes) = line.split_once("=").unwrap();
        let (left_raw, right_raw) = lr_nodes.split_once(",").unwrap();
        // Some cleaning
        key = key.trim();
        let left: String = left_raw.chars().filter( |n| !n.is_ascii_punctuation() && !n.is_whitespace()).collect();
        let right: String = right_raw.chars().filter( |n| !n.is_ascii_punctuation() && !n.is_whitespace()).collect();
        println!("Line: {} L:[{}] R:[{}]", line, left, right);
        map.insert(key.to_string(), Node::new(left, right));

        if key.chars().last().unwrap() == 'A' {
            starting_nodes.push(key);
        }
    }

    // For each node, find the cycle time, e.g. node AAZ lands on a XXZ node every 2304 times
    while let Some(node) = starting_nodes.pop() {
        println!("Node: {}", node);
        let turns = get_number_of_turns(&map, &lr_values, &node);
        println!("Turns: {}", turns);

    }


    //println!("Len: {}", navigation_path.len());
    
}
