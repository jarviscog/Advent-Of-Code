use std::fs;
use std::time::Instant;

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point {
            x,
            y,
        }
    }
}

fn shoelace(x_points: Vec<i64>, y_points: Vec<i64>) -> i64 {
    let mut total = 0;
    for i in 0..x_points.len() - 1{
        let lace_down = x_points[i] * y_points[i + 1];
        let lace_up = x_points[i + 1] * y_points[i];
        total += lace_down - lace_up;
    }
    total/2
}

fn main() {

    let now = Instant::now();

    let file_path = "input.txt";
    //let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut current_x = 0;
    let mut current_y = 0;
    let mut points: Vec<Point> = Vec::new();
    points.push(Point::new(0,0));
    for line in contents.lines() {

        let (_, rest) = line.split_once(' ').unwrap();
        let (_, color_str) = rest.split_once(' ').unwrap();

        let color = &color_str[2..8];
        //println!("Color: {}", &color[0..5]);
        let amount = u64::from_str_radix(&color[0..5], 16).unwrap();
        let direction: char = match color.chars().nth(5).unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!("Unexpected symbol")

        };
        //println!("{} {}", direction, amount);

        match direction {
            'U' => { current_y -= amount}
            'D' => { current_y += amount}
            'L' => { current_x -= amount}
            'R' => { current_x += amount}
            _ => panic!("Unexpected symbol")
        }

        points.push(Point::new(current_x as i64, current_y as i64));
    }
    
    let mut x_points = 0;
    let mut y_points = 0;
    for j in 0..points.len() - 1 {
        x_points += (points[j].x - points[j + 1].x).abs();
        y_points += (points[j].y - points[j + 1].y).abs();
    }
    let exterior_area: i64 = x_points + y_points;
    //println!("Ex: {}", exterior_area);

    // Use shoelace to get the area
    let mut x_points: Vec<i64> = Vec::new();
    let mut y_points: Vec<i64> = Vec::new();
    for point in points.iter(){
        x_points.push(point.x as i64);
        y_points.push(point.y as i64);
    }
    let interior_area = shoelace(x_points, y_points).abs() as i64;

    // Use picks therom to get the number
    let total_area = interior_area + exterior_area/2 + 1;

    let elapsed = now.elapsed();

    println!("Total area: {}", total_area);
    println!("Total time: {:.2?}", elapsed);



}
