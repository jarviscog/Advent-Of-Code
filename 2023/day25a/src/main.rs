use std::fs;



fn main() {

    //let file_path = "input.txt";
    let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut connections: Vec<(String, String)> = Vec::new();
    for line in contents.lines() {
        let (component, connecting_components) = line.split_once(":").unwrap();

        for conn_conponent in connecting_components.split(" ").into_iter().map(|s| s.trim()) {

            println!("{}, {}", component, conn_conponent);
            
            connections.push((String::from(component), String::from(conn_conponent)));
            connections.push((String::from(conn_conponent), String::from(component)));


        }
        

    }
}
