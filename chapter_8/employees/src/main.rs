use std::collections::HashMap;
use std::io;
use std::str::FromStr;

enum Choice {
    DepartmentsList = 1,
    EmployeesList = 2,
    AddEmployee = 3,
    Exit = 4,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(input: &str) -> Result<Choice, Self::Err> {
        match input {
            "1" => Ok(Choice::DepartmentsList),
            "2" => Ok(Choice::EmployeesList),
            "3" => Ok(Choice::AddEmployee),
            "4" => Ok(Choice::Exit),
            _ => Err(()),
        }
    }
}

macro_rules! clear_screen {
    () => {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    };
}

macro_rules! wait_for_enter {
    () => {
        println!("Press <Enter> to continue");

        let mut tmp = String::new();

        io::stdin().read_line(&mut tmp).expect("");
    };
}

fn print_menu() {
    clear_screen!();
    println!(
        "[{}] Print departments' list",
        Choice::DepartmentsList as u32
    );
    println!("[{}] Print employees' list", Choice::EmployeesList as u32);
    println!("[{}] Add employee", Choice::AddEmployee as u32);
    println!("[{}] Exit", Choice::Exit as u32);
}

fn print_departments(departments: &HashMap<String, Vec<String>>) {
    clear_screen!();

    if departments.len() == 0 {
        println!("Departments' list is empty");
    } else {
        println!("Departments' list:");

        for department in departments.keys() {
            println!("  {department} department");
        }
    }

    wait_for_enter!();
}

fn print_employee(departments: &HashMap<String, Vec<String>>) {
    clear_screen!();

    if departments.len() == 0 {
        println!("Employees' list is empty");
    } else {
        println!("Employees' list:");

        for department in departments.keys() {
            println!("  {department} department:");

            for employee in &departments[department] {
                println!("    {employee}");
            }
        }
    }

    wait_for_enter!();
}

fn add_employee(departments: &mut HashMap<String, Vec<String>>) -> Result<(), &str> {
    clear_screen!();
    println!("Input command:");

    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Error reading command");

    if command.to_ascii_lowercase().starts_with("add ") {
        let params: Vec<&str> = command.trim()[4..].split(" to ").collect();

        if params.len() != 2 {
            return Err("Invalid command format");
        }

        let user = String::from(params[0]);
        let department = String::from(params[1]);

        departments.entry(department).or_insert(vec![]).push(user);

        println!(
            "Employee \"{}\" successfully added to \"{}\" department",
            params[0], params[1]
        );
        wait_for_enter!();

        return Ok(());
    }

    Err("Invalid command format")
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut line: String = String::new();

        print_menu();

        io::stdin()
            .read_line(&mut line)
            .expect("Error reading choise");

        let choice: Choice = match Choice::from_str(line.trim()) {
            Ok(x) => x,
            Err(e) => {
                println!("{e:?}");
                continue;
            }
        };

        match choice {
            Choice::DepartmentsList => print_departments(&departments),
            Choice::EmployeesList => print_employee(&departments),
            Choice::AddEmployee => match add_employee(&mut departments) {
                Ok(_) => {
                    println!("Employee added successful!")
                }
                Err(e) => {
                    println!("{e}")
                }
            },
            Choice::Exit => break,
        }
    }
}