use std::fs;

fn recursive_generate(v: &Vec<i32>, level_no: u32) -> i32 {

    for _ in 0..level_no { print!("    "); }
    let first = v.first().unwrap();
    if v.iter().all(|n| n == first) {
        let mut ret = v.clone();
        ret.pop();
        return *first;
    } else {
        // If all of the elements are not the same, get the inner vec
        let mut inner_vec: Vec<i32> = Vec::new();
        for (i, n) in v.iter().enumerate() {
            if i == 0 {
                continue;
            }
            
            let diff = n - v.get(i-1).unwrap();
            print!("{:<8} ", diff);
            inner_vec.push(diff);
        }
        println!("");
        let below_number = recursive_generate(&inner_vec, level_no + 1);
        return v.first().unwrap() - below_number;
    }

}

fn main() {

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut ans = 0;
    for line in contents.lines() {

        let mut nums: Vec<i32> = Vec::new();
        for num in line.split(" ").map(|n| n.parse::<i32>().unwrap()) {
            nums.push(num);
            print!("{:<8} ", num);
        }
        println!("");
        let next_value = recursive_generate(&nums, 1);
        ans += next_value;
        println!("\nValue: {}\n\n", next_value);
    }

    println!("Ans: {}", ans);
}
