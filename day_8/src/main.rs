use std::fmt;
use std::fs::File;
use std::io::ErrorKind;

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
    let greeting_file = match greeting_file_error {
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
    let greeting_file = File::open("hello.txt").unwrap();
    // Or
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in the project.");

    // Propagating the error. (?) operator
    // Used at end of expression.
    // If the result is error, it will return the error, and if it is Ok, the value inside ok will be returned and the program will be continued.
    // The return type of function should be same, which ? will return.
    // The return type of main is ()
}
