use std::{collections::HashMap, io};

fn main() {
    println!(
"Hello, world!
Welcome to the employee software developed in Rust
Please use this software to enter employee's to the directory
and hash-map of employees within the software"
    );

    let mut company_map = HashMap::new();

    company_map.insert("engineering".to_string(), Vec::new());
    company_map.insert("operations".to_string(), Vec::new());
    company_map.insert("sales".to_string(), Vec::new());

    loop {
        println!(
"Begin input your command: 
'add (employee) to (department)' 
OR 
'view (department/all)'
OR
'quit' to leave application"
        );

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input) // put the input into the mutable variable guess and use a reference to allocate memory accordingly??
            .expect("Failed to read line");

        let user_command = user_input.split_whitespace().next().unwrap().to_lowercase();
        println!("User input: {}", user_input);
        println!("Command: {:?}", user_command);

        match user_input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["add", emp, "to", dep] => add_employee(&mut company_map, emp, dep),
            ["view", "department", dep] => view_department(&company_map, dep),
            ["view", "all"] => view_all(&company_map),
            ["quit"] => break,
            _ => println!("Please enter a valid option\n")
        }
    }
}

fn view_department(company: &HashMap<String, Vec<String>>, department: &str) {
    println!("Employees in {department}:\n");
    for e in &company[department] {
        println!("{e}\n");
    }
}

fn view_all(company_map: &HashMap<String, Vec<String>>) {
    for (department, employees) in company_map {
        println!("Department: {department}");
        for e in employees {
            println!("{e}\n");
        }
    }
}

fn add_employee(company_map: &mut HashMap<String, Vec<String>>, employee: &str, department: &str) {
    println!("{employee} added to {department}");

    company_map.entry(department.to_string()).or_insert(Vec::new());

    let vec = company_map.get_mut(department).unwrap();
    vec.push(employee.to_owned());
    vec.sort_by_key(|name| name.to_lowercase());
    println!("{company_map:?}\n")
}
