use core::panic;
use std::fs;
use std::collections::HashMap;
use std::time::Instant;
use std::ops::Range;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Category {
    Extreme,
    Musical,
    Aerodynamic,
    Shiny
}

#[derive(PartialEq, Clone)]
enum Condition {
    GreaterThan,
    LessThan
}

#[derive(Clone)]
struct Rule {
    category: Category,
    condition: Condition,
    amount: u32,
    dest: String
}

impl Rule {
    fn new(category: Category, condition: Condition, amount: u32, dest: String) -> Rule {
        Rule {
            category,
            condition,
            amount,
            dest,
        }
    }
}


struct Workflow {
    rules: Vec<Rule>,
    dest: String
}

impl Workflow {
    fn new(rules: Vec<Rule>, dest: String) -> Workflow {
        Workflow {
            rules,
            dest
        }
    }
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
   fn new(x: u32, m: u32, a: u32, s: u32) -> Part {
        Part {
            x,
            m,
            a,
            s
        }
   }

   fn total(&self) -> u32 {
       self.x + self.m + self.a + self.s
   }
}

fn is_accepted(part: &Part, map: &HashMap<String, Workflow>) -> bool {

    let mut current_workflow_name = String::from("in");
    while current_workflow_name != "A" && current_workflow_name != "R" {
        //println!("{}", current_workflow_name);
        let current_workflow: &Workflow = map.get(&current_workflow_name).unwrap();

        for rule in &current_workflow.rules {
            let part_amount = match rule.category {
                Category::Extreme => part.x,
                Category::Musical => part.m,
                Category::Aerodynamic => part.a,
                Category::Shiny => part.s,
            };
            if rule.condition == Condition::GreaterThan {
                if part_amount > rule.amount {
                    current_workflow_name = rule.dest.clone();
                    break;
                }
            } else {
                if part_amount < rule.amount {
                    current_workflow_name = rule.dest.clone();
                    break;
                }
            }

            current_workflow_name = current_workflow.dest.clone();
        }
    }

    if current_workflow_name == "A" {
        return true
    } else {
        return false;
    }
}


fn count(mut ranges: HashMap<Category, Range<u32>>, workflow_name: &str, map: &HashMap<String, Workflow>) -> usize {

    if workflow_name == "R" {
        return 0;
    } 
    if workflow_name == "A" {
        return 
            (ranges[&Category::Extreme].len() + 1) * 
            (ranges[&Category::Musical].len() + 1) * 
            (ranges[&Category::Aerodynamic].len() + 1) * 
            (ranges[&Category::Shiny].len() + 1) 
    }

    let workflow = map.get(workflow_name).unwrap().clone();
    let mut total = 0;

    let mut brk = false;
    
    for rule in workflow.rules.clone() {
        
        let low: u32 = ranges.clone()
            .get(&rule.category)
            .unwrap()
            .start.clone();
        let high: u32 = ranges.get(&rule.category.clone()).unwrap().clone().end.clone();
        let t: Range<u32>;
        let f: Range<u32>;
        match rule.condition {
            Condition::LessThan => {
                t = low..rule.amount - 1;
                f = rule.amount..high;
            }
            Condition::GreaterThan => {
                t = rule.amount + 1..high;
                f = low..rule.amount;
            }
        }
        if t.start <= t.end {
            let mut ranges_copy = ranges.clone();
            *ranges_copy.get_mut(&rule.category).unwrap() = t;
            total += count(ranges_copy, &rule.dest, map)
        }
        if f.start <= f.end {
            *ranges.get_mut(&rule.category).unwrap() = f;
        } else {
            brk = true;
            break;
        }
    } 
    if brk == false {
        total += count(ranges, &workflow.dest, map)
    }


    total
}

fn main() {

    let now = Instant::now();
    let file_path = "input.txt";
    //let file_path = "test_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let (workflows_str, parts_str) = contents.split_once("\n\n").unwrap();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for line in workflows_str.lines() {
        //println!("{}", line);
        let (name, rest) = line.split_once("{").unwrap();
        let rules_strings: Vec<&str> = rest.trim_end_matches("}").split(",").into_iter().collect();
        let mut dest: &str = "";

        let mut line_rules: Vec<Rule> = Vec::new();

        for rule in rules_strings {
            //println!("{}", rule);

            let category_str: &str;
            let condition: Condition;
            let amount_str: &str;

            if rule.contains(">") || rule.contains("<") {

                if rule.contains(">") {
                    (category_str, amount_str) = rule.split_once(">").unwrap();
                    condition = Condition::GreaterThan; 
                } else { // if rule.contains("<") 
                    (category_str, amount_str) = rule.split_once("<").unwrap();
                    condition = Condition::LessThan; 
                }

                let category = match category_str {
                    "x" => Category::Extreme,
                    "m" => Category::Musical,
                    "a" => Category::Aerodynamic,
                    "s" => Category::Shiny,
                    _ => panic!("Unexpeced symbol")
                };
                let (amount_str, rule_dest) = amount_str.split_once(":").unwrap();
                line_rules.push(
                    Rule::new(
                        category, 
                        condition, 
                        amount_str.parse().unwrap(),
                        String::from(rule_dest)
                        ))
            } else {
                dest = rule;
            }
            
        }
        workflows.insert(
            String::from(name),
            Workflow::new( line_rules, String::from(dest))
            );
    }
    
    let mut ranges: HashMap<Category, Range<u32>> = HashMap::new();
    ranges.insert(Category::Extreme, 1..4000);
    ranges.insert(Category::Musical, 1..4000);
    ranges.insert(Category::Aerodynamic, 1..4000);
    ranges.insert(Category::Shiny, 1..4000);

    let total = count(ranges, "in", &workflows);

    println!("{}", total);
    let duration = now.elapsed();
    println!("{:.2?}", duration);
}
