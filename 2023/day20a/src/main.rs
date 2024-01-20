use core::panic;
use std::fs;
use std::collections::{VecDeque, HashMap};

struct Pulse {
    is_high: bool,
    from: String,
    dest: String,
}

impl Pulse {
    fn new(is_high: bool, from: String, dest: String) -> Pulse {
        Pulse {
            is_high,
            from,
            dest
        }
    }
    fn to_string(&self) -> String {
        format!("{} --{:5}--> {}", self.from, self.is_high, self.dest)
    }
}

trait Module {
    fn get_name(&self) -> String;
    fn get_outputs(&self) -> Vec<String>;
    fn recieve_pulse(&mut self, pulse: Pulse) -> Vec<Pulse>;
    fn to_string(&self) -> String;
    fn add_connection(&mut self, name: String) {}
}

struct BroadcastModule {
   out_connections: Vec<String> 
}
impl BroadcastModule {
    fn new(out_connections: Vec<String>) -> BroadcastModule {
        BroadcastModule { out_connections }
    }
}

impl Module for BroadcastModule {

    fn get_name(&self) -> String {
        String::from("broadcaster")
    }
    fn get_outputs(&self) -> Vec<String> {
        self.out_connections.clone()
    }
    
    fn recieve_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let mut out_pulses: Vec<Pulse> = Vec::new();
        for connection in &self.out_connections {
            out_pulses.push(
                Pulse::new(pulse.is_high, self.get_name(), connection.clone())
                )
        }
        return out_pulses;
    }
    fn to_string(&self) -> String {
        let mut out_string = String::new();
        out_string += "broadcaster: ";
        for connection in &self.out_connections {
            out_string += &connection;
            out_string += ", ";
        }
        out_string
    }
}

struct FlipFlopModule {
    name: String,
    is_on: bool,
    dest: Vec<String>
}
impl FlipFlopModule {
    fn new(name: String, dest: Vec<String>) -> FlipFlopModule {
        let is_on = false;
        FlipFlopModule {
            name,
            is_on,
            dest 
        }

    }
}

impl Module for FlipFlopModule {

    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_outputs(&self) -> Vec<String> {
        self.dest.clone()
    }

    fn recieve_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let mut out_pulses: Vec<Pulse> = Vec::new();
        if !pulse.is_high {
            self.is_on = !self.is_on;
            for dest in &self.dest {
                out_pulses.push(
                    Pulse::new(
                        self.is_on,
                        self.get_name(),
                        dest.clone(),
                        )
                    )
            }
        } 
        return out_pulses;
    }
    fn to_string(&self) -> String {
        let mut out_string = String::new();
        out_string += "Flip Flop:   ";
        out_string += &self.name;
        out_string += " -> ";
        for dest in &self.dest {
            out_string += dest;
            out_string += " ";
        }
        out_string
    }
}

struct ConjunctionModule {
    name: String,
    from: HashMap<String, bool>,
    dest: Vec<String>
}
impl ConjunctionModule {
    fn new(name: String, dest: Vec<String>) -> ConjunctionModule {
        let from = HashMap::new();
        ConjunctionModule {
            name,
            from,
            dest
        }
    }
}

impl Module for ConjunctionModule {

    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_outputs(&self) -> Vec<String> {
        self.dest.clone()
    }

    fn recieve_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        //println!("Conjunction recieving pulse: {}", self.name);
        //println!("Connections: ");
        for key in self.from.keys() {
            let value: bool = self.from.get(key).unwrap().clone();
            //println!("  {}: {}", key, value);
        }

        if let Some(s) = self.from.get_mut(&pulse.from) {
            //println!("Updating");
            *s = pulse.is_high.clone();
        } else {
            //println!("Inserting");
            self.from.insert(pulse.from, pulse.is_high);
        }

        let mut all_true = true;
        for value in self.from.values() {
            //println!("Value: {}", value);
            if *value == false {
                all_true = false;
                break;
            }
        }

        let mut out_pulses: Vec<Pulse> = Vec::new();
        for name in &self.dest {
            out_pulses.push(
                Pulse::new(
                    !all_true,
                    self.get_name(),
                    name.clone(),
                    )
                )
        }

        out_pulses
    }
    
    fn to_string(&self) -> String {
        let mut out_string = String::new();
        out_string += "Conjunction: ";
        out_string += &self.name;
        out_string += " -> ";
        for connection in &self.dest {
            out_string += &connection;
            out_string += ", ";
        }
        out_string
    }
    fn add_connection(&mut self, name: String) {
        self.from.insert(name, false);
   }
}



fn main() {

    let file_path = "input.txt";
    //let file_path = "test_one_input.txt";
    //let file_path = "test_two_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    for line in contents.lines() {
        let (module_str, outputs_str) = line.split_once(" -> ").unwrap();

        let outputs: Vec<String> = outputs_str.split(",").map(|s| String::from(s.trim())).collect();

        match module_str.chars().nth(0).unwrap() {
            'b' => {
                modules.insert(
                    String::from("broadcaster"),
                    Box::from(BroadcastModule::new(outputs))
                    );
            }
            '%' => {
                let module_name = String::from(module_str.trim_start_matches("%"));
                modules.insert(
                    module_name.clone(),
                    Box::from(FlipFlopModule::new(module_name, outputs))
                    );
            }
            '&' => {
                let module_name = String::from(module_str.trim_start_matches("&"));
                modules.insert(
                    module_name.clone(),
                    Box::from(ConjunctionModule::new(module_name, outputs))
                    );
            }
            _ => panic!("Unexpected type")
        }
    }

    let keys = modules.keys().clone();

    let mut connections: Vec<(String, String)> = Vec::new();
    for module_name in keys {
        println!("{}", modules.get(module_name).unwrap().to_string());
        for output_module in modules.get(module_name).unwrap().get_outputs() {
            connections.push((module_name.clone(), output_module))
        }
    }

    for (from, to) in connections {
        if let Some(output) = modules.get_mut(&to) {
            output.add_connection(from.clone());
        }
        
    }

    let mut pulses: VecDeque<Pulse> = VecDeque::new();

    

    println!();
    let mut total_high = 0;
    let mut total_low = 0;
    for i in 0..1000000 {

        let button_press = Pulse::new( false, String::from(format!("Button {}", i)), String::from("broadcaster"),);
        pulses.push_back(button_press);

        while let Some(pulse_to_process) = pulses.pop_front() {
            //println!("{}", pulse_to_process.to_string());
            if !pulse_to_process.is_high && pulse_to_process.from == String::from("rx") {
                println!("Found it! {}", i);
                break;
            }

            if pulse_to_process.is_high {
                total_high += 1;
            } else {
                total_low += 1;
            }

            if let Some(dest_module) = modules.get_mut(&pulse_to_process.dest) {
                let new_pulses: Vec<Pulse> = dest_module.recieve_pulse( pulse_to_process );

                //println!("New pulses: ");
                for pulse in new_pulses {
                    pulses.push_back(pulse);
                }
            } else {
                //println!("Non-output: {}", pulse_to_process.dest)
            }


        }

        println!();
    }

    println!();

    println!("High: {} Low: {}", total_high, total_low);
    println!("Product: {}", total_high * total_low);




}
