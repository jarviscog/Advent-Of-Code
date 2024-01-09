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

fn main() {

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");
    let mut lines = contents.split("\n").filter(|x| !x.is_empty());
    let lr_values: String = lines.nth(0).unwrap().to_string(); // This pops the value
    lines.nth(0); // Pops a blank line
    
    let mut map: HashMap<String, Node> = HashMap::new();
    for line in lines {
        let (mut key, lr_nodes) = line.split_once("=").unwrap();
        let (left_raw, right_raw) = lr_nodes.split_once(",").unwrap();

        // Some cleaning
        key = key.trim();
        let left: String = left_raw.chars().filter( |n| !n.is_ascii_punctuation() && !n.is_whitespace()).collect();
        let right: String = right_raw.chars().filter( |n| !n.is_ascii_punctuation() && !n.is_whitespace()).collect();

        map.insert(key.to_string(), Node::new(left, right));
    }

    let mut navigation_path: String = String::new();
    let mut current_pos = 0;
    let mut current_node: String = String::from("AAA");
    while current_node != "ZZZ" {
        match lr_values.chars().nth(current_pos).unwrap() {
            'L' => {
                current_node = map.get(&current_node).unwrap().left.clone();
                navigation_path.push('L');
            }
            'R' => {
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
    println!("Len: {}", navigation_path.len());
    
}
