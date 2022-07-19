use std::collections::HashMap;
use std::{io};


fn get_user_input() -> String {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn add(departments: &mut HashMap<String, Vec<String>>, employee: &str,department_name: &str){

    match departments.get_mut(&String::from(department_name)) {
        Some(department) => {
            println!("Adding {employee} to {department_name}");
            department.push(String::from(employee));
        },
        None => {
            println!("Creating new department {department_name}, with {employee} as the first employee");
            departments.insert(String::from(department_name), vec![String::from(employee)]);
        },
    }
}

fn get(departments: &HashMap<String,Vec<String>>, department_name: &str){

    if department_name.to_lowercase() == "all" {

        let mut all_employees: Vec<String> = Vec::new();
        for (_, department) in departments {
            for employee in department {
                all_employees.push(employee.to_string());
            }
        }
        println!("All employees of company: {all_employees:?}");

    }else {

        match departments.get(&String::from(department_name)) {
            Some(result) => println!("Employees of department {department_name}: {result:?}"),
            None => {println!("Unknown department")},
        }

    }
}

fn command_parser(command : &String, departments: &mut HashMap<String, Vec<String>>){
    let args: Vec<&str> = command.split_whitespace().collect();
    if args.len() <= 0 {
        println!("No command found");
        return;
    }

    match args[0] {
        "add" | "Add" => {
            if args.len() != 4{
                println!("Wrong command syntax! Add <employee> to <department>");
                return;
            }
            add(departments,args[1],args[3]);
        },
        "get" => {
            get(departments, args[1])
        },
        _ => {println!("Unknown command");}
    }
}



fn main() {
    let mut departments = HashMap::new();

    loop {
        println!("Enter a command: ");
        let command = get_user_input();
        println!("Entered: {command}");
        if command.contains("quit")  {
            println!("Quitting program!");
            return;
        }else {
            command_parser(&command, &mut departments);
        }
    }

}
