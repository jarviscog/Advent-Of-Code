use std::fs;
use std::time::Instant;


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

    #[allow(dead_code)]
    fn to_string(&self) -> String{
        format!("{} {} {}", self.dest, self.source, self.range)
    }

}

fn map_number(maps: &Vec<Map>, num: &usize) -> usize {
    for map in maps {
        if *num as usize >= map.source && *num < map.source + map.range {
            return map.dest + *num - map.source;
        }
    }
    *num
}

fn main() {

    //let file_path = "test_input.txt";
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut maps: Vec<Vec<Map>> = Vec::new();
    
    // Wow, this is going to take a while to compute
    let (seeds_str, section_str) = contents.split_once("\n\n").unwrap();
    let mut seed_pairs: Vec<usize> = 
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

    let mut seeds: Vec<usize> = Vec::new();
    println!("Adding seeds...");
    while seed_pairs.len() > 0 {
        let seed_number = seed_pairs.remove(0);
        let range = seed_pairs.remove(0);

        // I am terrified of the implications of this for my RAM
        for i in 0..range {
            seeds.push(seed_number + i);
        }


    }
    let number_of_seeds = seeds.len();
    println!("Adding seeds...Done!");
    

    // Sort the contents of the string
    for section_str in section_str.split("\n\n") {
        
        let mut section_maps: Vec<Map> = Vec::new();
        let (_, section_data) = section_str.split_once(":").unwrap();

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
    
    println!("Number of seeds to compute: {number_of_seeds}"); 
    // This is very satisfying
    let number_of_maps = 7;
    for i in 0..number_of_maps {
        let before = Instant::now();
        for j in 0..number_of_seeds{
            let new_number = map_number(&maps[i], &seeds[j]); 
            seeds[j] = new_number;
        }
        println!("Done map {i} of {number_of_maps}", ); 
        println!("Elapsed time: {:.2?}", before.elapsed());

    }

    println!("Sorting...");
    let before = Instant::now();
    seeds.sort();
    println!("Sorting...Done!");
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("{}", seeds.first().unwrap());
    

}
