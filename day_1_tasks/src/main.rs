// Implement Display trait for a struct named Date

use std::fmt;

#[derive(Debug)]
struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Date is {0:0>2}/{1:0>2}/{2:0>2}",
            self.day, self.month, self.year
        )
    }
}

#[derive(Debug)]
struct QuadraticEquation {
    x: i32,
    y: i32,
    constant: i32,
}

impl fmt::Display for QuadraticEquation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut x_val = format!("{}x", self.x.to_string());
        let mut y_val = format!(" {}y", self.y.to_string());
        let mut constant_val = format!(" {}", self.constant.to_string());
        if self.x == 0 {
            x_val = String::from("");
        }
        if self.y > 0 {
            y_val = format!(" + {}y", self.y)
        } else if self.y == 0 {
            y_val = String::from("");
        }
        if self.constant > 0 {
            constant_val = format!(" + {}", self.constant)
        } else if self.constant == 0 {
            constant_val = String::from("");
        }
        if self.x == 0 && self.y == 0 && self.constant == 0 {
            write!(f, "The quadratic equation is 0")
        } else {
            write!(
                f,
                "The quadratic equation is {x_val}{y_val}{constant_val} = 0"
            )
        }
    }
}

fn main() {
    let today_date = Date {
        day: 14,
        month: 02,
        year: 2024,
    };
    println!("{}", today_date);
    println!("{:?}", today_date);
    println!("{:#?}", today_date);

    let my_equation = QuadraticEquation {
        x: 23,
        y: 72,
        constant: -31,
    };
    let my_equation2 = QuadraticEquation {
        x: 0,
        y: 0,
        constant: 0,
    };
    println!("{}", my_equation);
    println!("{:?}", my_equation);
    println!("{:#?}", my_equation);
    println!("{}", my_equation2);
    println!("{:?}", my_equation2);
    println!("{:#?}", my_equation2);
}
