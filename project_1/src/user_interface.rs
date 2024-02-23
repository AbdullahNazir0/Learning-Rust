use crate::item::Item;
use crate::order::Order;
use std::io;

const USERNAME: &str = "AbdullahNazir0";
const PASSWORD: &str = "123";

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape codes for clearing the screen
}

pub fn login_page() {
    loop {
        println!("-------------------------Login Page-------------------------");
        println!("Welcome User");
        println!("Please Enter Your Username: ");
        let mut input_username: String = String::new();
        let mut input_password: String = String::new();
        io::stdin()
            .read_line(&mut input_username)
            .expect("Cannot read line.");
        if input_username.trim() == USERNAME {
            println!("Please Enter Your Password: ");
            io::stdin()
                .read_line(&mut input_password)
                .expect("Cannot read line,");
            if input_password.trim() == PASSWORD {
                clear_console();
                home_page();
            } else {
                clear_console();
                println!("Incorrect Password");
            }
        } else {
            clear_console();
            println!("Incorrect Username");
        }
    }
}

fn home_page() {
    loop {
        println!("-------------------------Home Page-------------------------");
        println!("1: Take Order");
        println!("2: Show Total Bills");
        println!("3: Search a Bill");
        println!("4: Delete a Bill");
        println!("5: Exit Program");
        println!("Your Choice:");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read line.");

        // It will panic if error.
        // let option: u8 = input.trim().parse().expect("Enter a Valid Choice.");

        let option: u8 = match input.trim().parse() {
            Ok(choice) => choice,
            Err(_) => {
                clear_console();
                println!("Enter a Valid Choice.");
                continue;
            }
        };

        match option {
            1 => take_order(),
            2 => show_total_bills(),
            3 => search_bill(),
            4 => delete_bill(),
            5 => std::process::exit(0),
            _ => {
                clear_console();
                println!("Invalid Number, Choose a number between 1-5.")
            }
        }
    }
}

fn take_order() {
    println!("-------------------------Menu Page-------------------------");
    println!("Success");
}
fn show_total_bills() {
    println!("---------------------------Bills---------------------------");
    let item1 = Item {
        name: String::from("Pizza"),
        price: 5.50,
    };

    let item2 = Item {
        name: String::from("Burger"),
        price: 6.30,
    };

    let mut order1 = Order {
        order_number: 01,
        customer_name: String::from("Abdullah"),
        total_items: 2,
        items: Vec::new(),
        net_total: 5.50,
    };

    order1.items.push(item1);
    order1.items.push(item2);

    println!("{}", order1);
}
fn search_bill() {
    println!("Success");
}
fn delete_bill() {
    println!("Success");
}
