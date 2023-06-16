// Copied from https://dev.to/nickymeuleman/advent-of-code-2022-day-3-53dm
// However, I really tried to understand the solution, as I want to lean some more powerful Rust
// techniques.

pub fn part_one() -> u32 {

    let filepath = "day_three_part_one.txt";
    let input;
    match std::fs::read_to_string(filepath) {
        Ok(val) => input = val,
        Err(_) => {panic!("NO FILE FOUND")},
    }
    
    return input.lines().filter_map(|line| {
        // Filter
        let line = line.as_bytes();
        let (left, right) = line.split_at(line.len() / 2);
        // Map
        left.iter().find(|item| right.contains(item))
            .map(|item| match item {
                b'a'..=b'z' => (item - b'a') + 1,
                _ => (item - b'A') + 1 + 26,
            } as u32)
    }).sum();
}

