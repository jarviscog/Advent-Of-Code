use std::fs;


struct Map {
    dest: usize,
    source: usize,
    range: usize 
}

impl Map {

    fn new(dest: usize, source: usize, range: usize) -> Map {
        Map {
            dest,
            source,
            range 
        }
    }

    fn to_string(&self) -> String{
        format!("{} {} {}", self.dest, self.source, self.range)
    }

}

fn map_number(maps: &Vec<Map>, num: &usize) -> usize {

    let mut potential_nums: Vec<usize> = Vec::new();
    for map in maps {
        let sum: usize = map.source + map.range;
        println!("Is {} between {} and {} ", num, map.source, sum - 1);
        if *num as usize >= map.source && *num < map.source + map.range {
            println!("Hit: {} on map {}", num, map.to_string());
            // NOTE: This section has major implications on the output. If multiple maps in a
            // section apply to a number, this will be wrong
            let ans = (map.dest + (*num as usize - map.source)) as usize;
            return ans;
            //potential_nums.push();
        }
    }
    
    potential_nums.sort();
    return match potential_nums.first() {
        Some(n) => *n,
        None => *num,
    }

}

fn main() {

    //let file_path = "test_input.txt";
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut maps: Vec<Vec<Map>> = Vec::new();
    
    let (seeds_str, section_str) = contents.split_once("\n\n").unwrap();
    let mut seeds: Vec<usize> = 
       seeds_str 
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    
    let number_of_seeds = seeds.len();

    // Sort the contents of the string
    for section_str in section_str.split("\n\n") {
        
        let mut section_maps: Vec<Map> = Vec::new();
        let (section_name, section_data) = section_str.split_once(":").unwrap();

        println!("Section name: {}", section_name);
        for line in section_data.split("\n").filter(|s| !s.is_empty()) {
            // A bit confusing, but nth pops the value when used
            let mut line_nums = line.split_whitespace().map(|n| n.parse::<usize>().unwrap());
            let dest = line_nums.nth(0).unwrap();
            let source = line_nums.nth(0).unwrap();
            let range = line_nums.nth(0).unwrap();

            section_maps.push(Map::new(dest, source, range));
        }

        maps.push(section_maps);

    }
    
    
    println!("Seeds: ");
    for seed in &seeds {
        println!("{}", seed);
    }
    println!("Sections: ");
    for section in &maps {
        println!("");
        for map in section {
            println!("{}", map.to_string());
        }
    }

    // This is very satisfying
    let number_of_maps = 7;
    for i in 0..number_of_maps {
        for j in 0..number_of_seeds{
            let new_number = map_number(&maps[i], &seeds[j]); 
            println!("{} -> {}", seeds[j], new_number);
            seeds[j] = new_number;
        }

    }

    println!("");
    println!("After: ");
    seeds.sort();
    for seed in seeds {
        println!("{}", seed);
    }
    

}
