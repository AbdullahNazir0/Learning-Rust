use std::fmt;
use std::fs::File;
use std::io::ErrorKind;

mod tasks;

#[derive(Debug)]
struct MyStruct {
    field1: String,
    field2: u32,
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is my struct: {}, {}", self.field1, self.field2)?;
        write!(f, "This is second line")?;
        Ok(())
    }
}

fn main() {
    let greeting_file_error = File::open("hello.txt");
    let _greeting_file = match greeting_file_error {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem Encountered while Creating the File: {:?}", e),
            },
            other_error => panic!("Problem Opening the file: {:?}", other_error),
        },
    };

    // Or
    let _greeting_file = File::open("hello.txt").unwrap();
    // Or
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in the project.");

    // Propagating the error. (?) operator
    // Used at end of expression.
    // If the result is error, it will return the error, and if it is Ok, the value inside ok will be returned and the program will be continued.
    // The return type of function should be same, which ? will return.
    // The return type of main is ()

    // -------------->
    // Task1
    let my_result = tasks::divide(6, 0);
    match my_result {
        Ok(num) => println!("The result is {}", num),
        Err(err) => println!("Cannot divide: {:?}", err),
    }
    // Task2
    let my_option: Option<u32> = Some(5);
    let my_result: Result<u32, &str> = Ok(5);
    tasks::check_option_result(my_option, my_result);
    // Task3
    println!("{:?}", tasks::check_custom_error());
    // Task4
    println!("{}", tasks::input_valid_integer());
}
