// // fmt::Debug
// #[allow(dead_code)]
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: i32,
//     address: String,
// }

// // ----------------------------------------------------------------->

// // fmt::Display
// use std::fmt; // fmt is a module in std library.
// struct MyStruct {
//     parameter_1: String,
//     parameter_2: i32,
//     parameter_3: bool,
// }
// // Manual implementation of trait fmt::Display to use marker '{}'
// impl fmt::Display for MyStruct {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "Parameter1(String): {0}\nParameter2(i32): {1}\nParameter3(bool): {2}",
//             self.parameter_1, self.parameter_2, self.parameter_3
//         )
//     }
// }

// fn main() {
//     let person1 = Person {
//         name: String::from("Abdullah"),
//         age: 19,
//         address: String::from("No address"),
//     };
//     println!("{:?}", person1);
//     println!("{:#?}", person1);

//     // ----------------------------------------------------------------->

//     let struct1 = MyStruct {
//         parameter_1: String::from("String_Parameter"),
//         parameter_2: 10,
//         parameter_3: true,
//     };

//     println!("{}", struct1);
// }

// <------------------------------------------------------------------------------------------------------------>
// Activity.

use std::fmt;

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hex_red = format!("{:X}", self.red);
        let hex_green = format!("{:X}", self.green);
        let hex_blue = format!("{:X}", self.blue);
        write!(
            f,
            "RGB ({0}, {1}, {2}) 0x{hex_red:0>2}{hex_green:0>2}{hex_blue:0>2}",
            self.red, self.green, self.blue
        )
    }
}

fn main() {
    let colors = [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ];

    for color in colors {
        println!("{}", color);
    }
}
