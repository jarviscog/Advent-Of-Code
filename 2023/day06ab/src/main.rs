use std::fs;

struct Race {
    time: usize,
    distance: usize
}

impl Race {

    fn new(time: usize, distance: usize) -> Race {
        Race {
            time,
            distance
        }
    }

    fn to_string(&self) -> String {
       format!("Time: {} Distance: {}", self.time, self.distance) 
    }


}
fn main() {

    //let file_path = "test_input.txt";
    //let file_path = "input_part_one.txt";
    let file_path = "input_part_two.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Coud not read file");

    let times: Vec<usize> = 
        contents.lines().nth(0).unwrap().split(":").nth(1).unwrap()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = 
        contents.lines().nth(1).unwrap().split(":").nth(1).unwrap()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    let mut races: Vec<Race> = Vec::new();
    for i in 0..times.len() {
        races.push(Race::new(times[i], distances[i]))
    }
    let mut product = 1;

    for race in races {
        println!("{}", race.to_string());
        let mut current_hold_time = 0;
        let mut temp_time = 0;
        while temp_time <= race.distance {
            current_hold_time += 1;
            temp_time = current_hold_time * (race.time - current_hold_time);
        }
        let min_time = current_hold_time;
        current_hold_time = race.time;
        let mut temp_time = 0;
        while temp_time <= race.distance {
            current_hold_time -= 1;
            temp_time = current_hold_time * (race.time - current_hold_time);
        }
        let max_time = current_hold_time;

        product *= max_time - min_time + 1;

        println!("Min: {}", min_time);
        println!("Max: {}", max_time);
        println!("Diff: {}", max_time - min_time + 1);

    }
    
    println!("Product: {}", product);

}

