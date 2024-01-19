use std::fs;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
        }
    }
}

fn shoelace(x_points: Vec<i32>, y_points: Vec<i32>) -> i32 {
    let mut total = 0;
    for i in 0..x_points.len() - 1{
        let lace_down = x_points[i] * y_points[i + 1];
        let lace_up = x_points[i + 1] * y_points[i];
        total += lace_down - lace_up;
    }
    total/2
}

fn main() {

    let file_path = "input.txt";
    //let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut current_x = 0;
    let mut current_y = 0;
    let mut points: Vec<Point> = Vec::new();
    points.push(Point::new(0,0));
    for line in contents.lines() {

        let (direction_str, rest) = line.split_once(' ').unwrap();
        let direction = direction_str.chars().nth(0).unwrap();
        let (amount_str, _) = rest.split_once(' ').unwrap();

        let amount: i32 = amount_str.parse::<i32>().unwrap();

        match direction {
            'U' => { current_y -= amount}
            'D' => { current_y += amount}
            'L' => { current_x -= amount}
            'R' => { current_x += amount}
            _ => panic!("Unexpected symbol")
        }
        points.push(Point::new(current_x, current_y));
    }
    
    let mut x_points = 0;
    let mut y_points = 0;
    for j in 0..points.len() - 1 {
        x_points += (points[j].x - points[j + 1].x).abs();
        y_points += (points[j].y - points[j + 1].y).abs();
    }
    let exterior_area: u32 = x_points as u32 + y_points as u32;

    // Use shoelace to get the area
    let mut x_points: Vec<i32> = Vec::new();
    let mut y_points: Vec<i32> = Vec::new();
    for point in points.iter(){
        x_points.push(point.x as i32);
        y_points.push(point.y as i32);
    }
    let interior_area = shoelace(x_points, y_points).abs() as u32;

    // Use picks therom to get the number
    let total_area = interior_area + exterior_area/2 + 1;
    println!("Total area: {}", total_area);


}
