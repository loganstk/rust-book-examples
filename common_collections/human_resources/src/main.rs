use std::io;
use std::collections::HashMap;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::str::FromStr;

#[derive(EnumIter, Eq, Hash, PartialEq, Debug)]
enum Department {
    Sales,
    Engineering,
    HumanResources,
    Finance,
}

impl FromStr for Department {
    type Err = ();

    fn from_str(input: &str) -> Result<Department, Self::Err> {
        match input {
            "Sales" => Ok(Department::Sales),
            "Engineering" => Ok(Department::Engineering),
            "HumanResources" => Ok(Department::HumanResources),
            "Finance" => Ok(Department::Finance),
            _ => Err(()),
        }
    }

}

#[derive(Debug)]
enum Command {
    Add { employee: String, department: Department },
    Remove { employee: String, department: Department },
    Transfer { employee: String, from: Department, to: Department },
    ListEmployees { department: Department },
}

fn main() {
    let mut organization: HashMap<Department, Vec<String>> = HashMap::new();

    init_org(&mut organization);

    loop {
        let mut user_input = String::new();
        io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
        let user_input = String::from(user_input.trim());
        let command = parse_command(user_input);

        match command.expect("Unknown command") {
            Command::Add { employee, department } => add_employee(&mut organization, department, employee),
            Command::Remove { employee, department } => remove_employee(&mut organization, department, employee),
            Command::Transfer { employee, from, to } => transfer_employee(&mut organization, from, to, employee),
            Command::ListEmployees { department } => list_deparment(&organization, department),
        }
    }

}

fn init_org(org: &mut HashMap<Department, Vec<String>>) {
    for department in Department::iter() {
        org.insert(department, Vec::new());
    }
}

fn add_employee(org: &mut HashMap<Department, Vec<String>>, department: Department, name: String) {
    let mut dep = org.get_mut(&department).unwrap();

    dep.push(name);
}

fn remove_employee(org: &mut HashMap<Department, Vec<String>>, department: Department, name: String) {
    let mut dep = org.get_mut(&department).unwrap();

    let position = dep.iter().position(|x| x == &name);

    dep.remove(position.expect("No such employee in the department."));
}

fn transfer_employee(org: &mut HashMap<Department, Vec<String>>, from: Department, to: Department, name: String) {
    remove_employee(org, from, name.clone());
    add_employee(org, to, name);
}

fn list_deparment(org: &HashMap<Department, Vec<String>>, department: Department) {
    let dep = org.get(&department).unwrap();

    println!("People working at the {:?} department: {:#?}", department, dep);
}

fn parse_command(input: String) -> Option<Command> {
    let input: Vec<&str> = input.as_str().split(' ').collect();

    let action = input[0];
    let employee = String::from(input[1]);

    match action {
        "Add" => Some(Command::Add {
            employee,
            department: Department::from_str(input[3]).expect("Unknown department!"),
        }),
        "Remove" => Some(Command::Remove {
            employee,
            department: Department::from_str(input[3]).expect("Unknown department!"),
        }),
        "Transfer" => Some(Command::Transfer {
            employee, 
            from: Department::from_str(input[3]).expect("Unknown department!"),
            to: Department::from_str(input[5]).expect("Unknown department!"),
        }),
        "List" => Some(Command::ListEmployees {
            department: Department::from_str(input[1]).expect("Unknown department!"),
        }),
        _ => None,
    }
}
