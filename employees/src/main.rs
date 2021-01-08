// This is hard to read
use std::collections::HashMap;
use std::io;

fn main() {

    let mut departaments = HashMap::new();

    println!("Say \"exit\" to get out!");

    loop {

        let mut string = String::new();
        io::stdin()
            .read_line(&mut string)
            .expect("Error while reading stdin!");

        let string = string.trim();

        // Let user exit out of loop
        if string == "exit".to_string() { break }
        if string == "".to_string() { continue }

        if &string.len() < &4_usize {
            println!("Invalid command!");
            continue;
        }

        // Add mode
        if (&string[..3] == "Add" || &string[..3] == "add") && &string[3..4] == " "  {

            // Get name of employee to add
            let name = &string[4..];
            let mut name_length = 0;

            for c in name.to_string().chars() {
                if c == ' ' { break }
                name_length += 1;
            }

            let string = &name[name_length..];
            let name = &name[..name_length];

            // Insert employee name into departament
            if !&string.is_empty() && &string[..4] == " to "  {
                let departament = (&string[4..]).to_string();
                let map = departaments.entry(departament).or_insert(Vec::<String>::new());

                map.push(name.to_string());
            }
        }

        // Show mode
        else if (&string[..4] == "Show" || &string[..4] == "show") && &string[4..5] == " " {

            let departament = &string[5..];
            if !&departament.is_empty() {
                if departament == "*" {
                    for (key, value) in &departaments {
                        println!("Departament: {}", key);

                        print!("Employees: ");
                        for e in value {
                            print!("{} ", e);
                        }
                        println!("");
                    }
                }
                else {
                    let employees = departaments.get(&departament.to_string()).unwrap();
                    println!("Departament: {}", departament);

                    print!("Employees: ");
                    for e in employees {
                        print!("{} ", e);
                    }
                    println!("");
                }
            }
        }

        else {
            println!("Invalid command!");
        }
    }
}

