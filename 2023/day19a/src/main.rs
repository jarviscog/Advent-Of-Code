use core::panic;
use std::fs;
use std::collections::HashMap;

enum Category {
    Extreme,
    Musical,
    Aerodynamic,
    Shiny
}

impl Category {
    fn to_char(&self) -> char {
        match self {
           Self::Extreme => 'x',
           Self::Musical => 'm',
           Self::Aerodynamic => 'a',
           Self::Shiny => 's',
        }
    }
}

#[derive(PartialEq, Eq)]
enum Condition {
    GreaterThan,
    LessThan
}

impl Condition {
    fn to_char(&self) -> char {
        match self {
           Self::GreaterThan => '>',
           Self::LessThan => '<',
        }
    }
}

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
    name: String,
    rules: Vec<Rule>,
    dest: String
}

impl Workflow {
    fn new(name: String, rules: Vec<Rule>, dest: String) -> Workflow {
        Workflow {
            name,
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

fn main() {

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
            Workflow::new(String::from(name), 
                          line_rules, 
                          String::from(dest)
                          )
            );
    }
    

    let mut total = 0;
    for part_str in parts_str.lines() {
        //println!("{}", part_str);
        let (x_str, rest) = part_str.split_once(",").unwrap();
        let (m_str, rest) = rest.split_once(",").unwrap();
        let (a_str, s_str) = rest.split_once(",").unwrap();
        let x = x_str.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap();
        let m = m_str.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap();
        let a = a_str.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap();
        let s = s_str.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap();
        let part = Part::new(x, m, a, s);

        println!("{} {} {} {}", part.x, part.m, part.a, part.s);
        if is_accepted(&part, &workflows) {
            println!("{}", part.total());
            total += part.total();
        }
    }
    println!("{}", total);

}
