use std::fmt;
use std::io;
// Write a function that takes two integers as input and returns the result of their division. However, handle the case where the second integer is zero by returning a Result with an appropriate error variant.
pub fn divide(num1: u32, num2: u32) -> Result<f64, String> {
    if num2 == 0 {
        return Err(String::from("Denominator cannot be 0."));
    }
    Ok(num1 as f64 / num2 as f64)
}

// Create a function that takes an Option and a Result as parameters. If the Option is None, panic with a custom error message. If the Result is an Err, print the error message; otherwise, print the result.
pub fn check_option_result(option: Option<u32>, result: Result<u32, &str>) {
    match option {
        Some(num) => println!("Option is : {}", num),
        None => panic!("Option is None."),
    }
    match result {
        Ok(res) => println!("The result is : {}", res),
        Err(err) => println!("The error is : {}", err),
    }
}

// Define a custom error type called MyError and use it in a function that may return a result. Create instances of your custom error in specific error conditions.
#[derive(Debug)]
pub struct CustomError {
    details: String,
}

impl CustomError {
    pub fn new(details: &str) -> CustomError {
        CustomError {
            details: details.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for CustomError {}

pub fn check_custom_error() -> Result<(), CustomError> {
    let num = 6;
    if num == 5 {
        Ok(())
    } else {
        Err(CustomError::new("This is the custom error."))
    }
}

// Implement a loop that continues to prompt the user for input until a valid integer is provided. Use Result to handle errors when parsing the input.
pub fn input_valid_integer() -> i32 {
    loop {
        let mut input: String = String::new();
        println!("Enter an Integer: ");
        io::stdin().read_line(&mut input).expect("Cannot read line");
        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(err) => println!("Enter a valid integer: {:?}", err),
        };
    }
}
