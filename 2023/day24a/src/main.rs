use std::{fs, i64};

struct IntersectionPoint {
    x: i64,
    y: i64
}

impl IntersectionPoint {
    fn new(x: i64, y: i64) -> IntersectionPoint {
        IntersectionPoint {
            x,
            y
        }
    }
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> Point {
        Point {
            x,
            y,
            z,
            vx,
            vy,
            vz
        }
    }
    fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
    fn find_intersection(p1: &Point, p2: &Point) -> IntersectionPoint {

        println!("{}", p1.to_string());
        println!("{}", p2.to_string());

        // y = mx + b
        let m1 = p1.vy/p1.vx;
        let m2 = p2.vy/p2.vx;

        println!("Formula 1: y = {}x + {}", m1, -p1.y);
        println!("Formula 2: y = {}x + {}", m2, -p2.y);

        let y_intercept = 
        //let x_intercept = 

        let intersection = IntersectionPoint::new(0,0);

        return intersection;
    }

}

fn main() {

    //let file_path = "input.txt";
    let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut points: Vec<Point> = Vec::new();
    for line in contents.lines() {
        println!("{}", line);
        
        let (position_string, velocity_string) = line.split_once("@").unwrap();

        let mut position_values= position_string.split(",");
        let x: f64 = position_values.nth(0).unwrap().trim().parse().unwrap();
        let y: f64 = position_values.nth(0).unwrap().trim().parse().unwrap();
        let z: f64 = position_values.nth(0).unwrap().trim().parse().unwrap();

        let mut velocity_values = velocity_string.split(",");
        let vx: f64 = velocity_values.nth(0).unwrap().trim().parse().unwrap();
        let vy: f64 = velocity_values.nth(0).unwrap().trim().parse().unwrap();
        let vz: f64 = velocity_values.nth(0).unwrap().trim().parse().unwrap();

        points.push(Point::new(x, y, z, vx, vy, vz));

    }

    // For test data
    let test_lower_bound = 7;
    let test_higher_bound = 27;

    // For actual data
    //let test_lower_bound = 200000000000000;
    //let test_higher_bound = 400000000000000;

    for j in 0..points.len() {
        for i in j + 1..points.len() {
            let intersection: IntersectionPoint = Point::find_intersection(&points[j], &points[i]);

            if (intersection.x > test_lower_bound) && (intersection.y > test_lower_bound)
                && intersection.x < test_higher_bound && intersection.y < test_higher_bound {
                    

                }
        }
    }

    for point in points {
        println!("{}", point.to_string());
    }
}
