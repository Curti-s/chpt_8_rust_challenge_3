// Using a hash map & vectors, create a text interface to allow a user to add
// employee names to a department in a company.
// eg. "Add Sally to Engineering" or  "Add Amir to Sales"
// They let the user retrieve a list of all people in a department or all
// people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

// required trait for lines
use std::io::BufRead;

enum Command {
    Add { dept:String, name:String }, // classic rust style struct
    List(String), // tuple struct
    All,
    Quit,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();

        // slice destructuring / slice pattern matching
        match words.as_slice() {
            ["All"] => Some(Command::All),
            ["Quit"] => Some(Command::Quit),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["Add", name, "to", dept] => Some(Command::Add { dept:dept.to_string(), name:name.to_string()}),
            _ => None,
        }
    }
}

fn main() {
    let mut employees: HashMap<String,Vec<String>> = HashMap::new();
    
    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");

    for line in io::stdin().lock().lines() {
        let input = line.expect("Failed to read user input");

        match Command::from_input(&input) {
            Some(Command::Add { dept, name }) => employees.entry(dept).or_default().push(name),
            Some(Command::List(dept)) => match employees.get(&dept) {
                Some(names) => {
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                },
                None => println!("I don't recognize that department"),
            },
            Some(Command::All) => {
                for (dept, names) in &employees {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
            },
            Some(Command::Quit) => break,
            None => println!("Input Error"),
        }
    }
    println!("Have a nice day");
}
