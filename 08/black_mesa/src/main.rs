use std::io;
use std::collections::HashMap;
use std::io::Write;
use std::{thread, time};

fn add_employee(d: &Vec<String>, e: &mut HashMap<String, String>) {
    loop {
        print!("{}[2J", 27 as char);
        println!("------------------------------------------------------------");
        println!("|     Black Mesa Research Facility - New Employee Entry    |");
        println!("------------------------------------------------------------");
        println!("");

        let mut new_employee_name = String::new();

        print!("(Please enter the name of the new employee) > ", );
        io::stdout().flush();
        io::stdin().read_line(&mut new_employee_name).expect("Failed to read line");

        print!("{}[2J", 27 as char);
        println!("OK! Please enter the department this employee belongs to");
        println!("");
        println!("Please choose from the following:", );
        println!("> 1 - {}", d[0]);
        println!("> 2 - {}", d[1]);
        println!("> 3 - {}", d[2]);

        let mut new_employee_department = String::new();

        print!("(Select a number) > ", );
        io::stdout().flush();
        io::stdin().read_line(&mut new_employee_department).expect("Failed to read line");

        let new_employee_department = match new_employee_department.trim().parse::<i32>().unwrap() {
                1 => &d[0],
                2 => &d[1],
                3 => &d[2],
                _ => {
                    println!("", );
                    println!("Sorry, Department not recognized - please try again ...", );
                    thread::sleep_ms(3000);
                    continue
                }
            };

        print!("{}[2J", 27 as char);
        thread::sleep_ms(2000);
        println!("", );
        println!("OK! Please please confirm your entry",);
        println!("");
        println!("Name - {}", new_employee_name);
        println!("Department - {}", new_employee_department);
        println!("");

        let mut confirm = String::new();

        print!("(y/n) > ", );
        io::stdout().flush();
        io::stdin().read_line(&mut confirm).expect("Failed to read line");

        let confirm = match confirm.trim() {
            "y" | "Y" => {
                print!("{}[2J", 27 as char);
                println!("Saving ...", );
                thread::sleep_ms(5000);
                print!("{}[2J", 27 as char);
                println!("OK! Entry has been saved ...", );
                thread::sleep_ms(3000);
                print!("{}[2J", 27 as char);
                println!("Returning to main menu ...", );
                thread::sleep_ms(3000);
                e.insert(new_employee_name.trim().to_string(), new_employee_department.to_string());
                thread::sleep_ms(3000);
                return }
            "n" | "N" => {
                println!("", );
                println!("You have selected N, entry wont be saved ...", );
                println!("Returning to main menu ...", );
                thread::sleep_ms(3000);
                break
                },
                _ => continue,
            };
        thread::sleep_ms(5000);
    }
}

fn get_employees(e: &HashMap<String, String>) {
    print!("{}[2J", 27 as char);
    println!("------------------------------------------------------------");
    println!("|         Black Mesa Research Facility - Employees         |");
    println!("------------------------------------------------------------");

    for (key, value) in e {
        println!("> [ {} ] - [ {} ]", key, value);
    }
    thread::sleep_ms(5000);
}

fn get_deparments(d: &Vec<String>) {
    print!("{}[2J", 27 as char);
    println!("------------------------------------------------------------");
    println!("|        Black Mesa Research Facility - Departments        |");
    println!("------------------------------------------------------------");

    for department in d.iter() {
        println!("> [ {} ]", department);
    }
    thread::sleep_ms(5000);
}

fn status() {
    print!("{}[2J", 27 as char);
    println!("------------------------------------------------------------");
    println!("|       Black Mesa Research Facility - Current Status       |");
    println!("------------------------------------------------------------");
    println!("> ...... WARNING ......", );
    println!("> ...... WARNING ......", );
    println!("> ...... WARNING ......", );
    println!("", );
    println!(" CATOSTROPHIC FAILURE DETECTED IN :", );
    println!("", );
    println!(" [ Sector F Lambda Complex ]", );
    println!("", );
    println!(" !!! HAZARDOUS ENVIRONMENT COMBAT UNIT HAVE BEEN DISPATCHED !!! ", );
    println!("", );
    println!(" !!! ALL PERSONNEL ARE REQUIRED TO EVACUATE IMMEDIATELY !!! ", );
    println!("", );
    thread::sleep_ms(10000);
}

fn loading() {
    print!("{}[2J", 27 as char);
    thread::sleep_ms(2000);
    println!("[ LOADING .                                                 ]", );
    thread::sleep_ms(1000);
    print!("{}[2J", 27 as char);
    println!("[ LOADING ..                                                ]", );
    thread::sleep_ms(1000);
    print!("{}[2J", 27 as char);
    println!("[ LOADING ...                                               ]", );
    thread::sleep_ms(1000);
    print!("{}[2J", 27 as char);
    println!("[ LOADING ....                                              ]", );
    thread::sleep_ms(2000);
    print!("{}[2J", 27 as char);
    println!("[ LOADING .....                                             ]", );
    thread::sleep_ms(1000);
    print!("{}[2J", 27 as char);
    println!("[ LOADING ......                                            ]", );
    print!("{}[2J", 27 as char);
    thread::sleep_ms(500);
    println!("[ LOADING ......                                            ]", );
    println!("[ *>                                                        ]", );
    print!("{}[2J", 27 as char);
    thread::sleep_ms(500);
    println!("[ LOADING ......                                            ]", );
    println!("[ **>                                                       ]", );
    print!("{}[2J", 27 as char);
    thread::sleep_ms(1000);
    println!("[ LOADING ......                                            ]", );
    println!("[ ****>                                                     ]", );
    print!("{}[2J", 27 as char);
    println!("[ LOADING ......                                            ]", );
    println!("[ ********>                                                 ]", );
    thread::sleep_ms(1000);
    print!("{}[2J", 27 as char);
    println!("[ LOADING ......                                            ]", );
    println!("[ **************>                                           ]", );
    thread::sleep_ms(2000);
    print!("{}[2J", 27 as char);
    println!("[ LOADING ......                                            ]", );
    println!("[ *****************>                                        ]", );
    thread::sleep_ms(2000);
    print!("{}[2J", 27 as char);
    println!("[ LOADING ......                                            ]", );
    println!("[ *******************************>                          ]", );
    thread::sleep_ms(5000);
    print!("{}[2J", 27 as char);
    println!("[ LOADING ......                                            ]", );
    println!("[ ************************************************>         ]", );
    thread::sleep_ms(1000);
    print!("{}[2J", 27 as char);
    println!("[ LOADING ......                                            ]", );
    println!("[ *******************************************************>  ]", );
    print!("{}[2J", 27 as char);
    println!("[ LOADING ......                                            ]", );
    println!("[ *********************************************************>]", );
    thread::sleep_ms(5000);
    println!("[ SUCCESSFULLY LOADED!                                      ]", );
    thread::sleep_ms(5000);
}

fn main() {
    let departments = vec![
        String::from("Administrative"),
        String::from("Science"),
        String::from("Security"),
    ];

    let mut employees: HashMap<String, String> = HashMap::new();
    employees.insert(String::from("Wallace Breen"), departments[0].to_string());
    employees.insert(String::from("Gordon Freeman"), departments[1].to_string());
    employees.insert(String::from("Barney Calhoun"), departments[2].to_string());

    print!("{}[2J", 27 as char);
    println!("------------------------------------------------------------");
    println!("|    Welcome to the Black Mesa Employee Resource Portal    |");
    println!("------------------------------------------------------------");
    thread::sleep_ms(2000);

    loading();

    loop {
        print!("{}[2J", 27 as char);
        println!("------------------------------------------------------------");
        println!("|         Black Mesa Research Facility - Main Menu          |");
        println!("------------------------------------------------------------");
        println!("> 1 - View Current Departments", );
        println!("> 2 - View Current Employees", );
        println!("> 3 - Add new Employee", );
        println!("> 4 - Check Status", );
        println!("> 5 - Quit", );
        println!("------------------------------------------------------------", );

        let mut x = String::new();
        print!("> ", );
        io::stdout().flush();
        io::stdin().read_line(&mut x).expect("Failed to read line");

        match x.trim().parse::<i32>().unwrap() {
            1 => get_deparments(&departments),
            2 => get_employees(&employees),
            3 => add_employee(&departments, &mut employees),
            4 => status(),
            5 => {
                thread::sleep_ms(1000);
                println!("> That's all for now ...", );
                thread::sleep_ms(1000);
                print!("{}[2J", 27 as char);
                thread::sleep_ms(1000);
                return
            }
            _ => continue,
        };
    }
}
