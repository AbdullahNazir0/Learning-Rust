// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// pub fn multiply(left: usize, right: usize) -> usize {
//     left * right
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
//     // fn can_hold(&self, other: &Rectangle) -> bool {
//     //     self.width < other.width && self.height > other.height
//     // }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn sum_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn multiply_works() {
//         let result = multiply(2, 5);
//         assert_eq!(result, 10);
//     }

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(!smaller.can_hold(&larger));
//     }
// }

// --------------->
// Tasks
pub fn calculate_area(len: isize, wid: isize) -> isize {
    len * wid
}

struct User {
    name: String,
    email: String,
    age: usize,
}

fn validate_email(user: &User) -> bool {
    if user.email.contains("@") && user.email.contains(".com") {
        true
    } else {
        false
    }
}

fn compare_ages(user1: &User, user2: &User) -> bool {
    user1.age == user2.age
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn positive_ok() {
        let res = calculate_area(8, 4);
        assert_eq!(res, 32, "Result should be 32, but is {}", res);
    }

    #[test]
    fn negative_ok() {
        let res = calculate_area(-8, -4);
        assert_eq!(res, 32, "Result should be 32, but is {}", res);
    }

    #[test]
    fn zero_ok() {
        let res = calculate_area(0, 0);
        assert_eq!(res, 0, "Result should be 32, but is {}", res);
    }

    #[test]
    fn user_ok() {
        let user = User {
            name: "Abdullah".to_owned(),
            email: "Abdullah.nazir289@gmail.com".to_owned(),
            age: 19,
        };
        let user2 = User {
            name: "Rust".to_owned(),
            email: "RustABC".to_owned(),
            age: 21,
        };
        assert!(validate_email(&user), "User email is not valid.");
        assert!(validate_email(&user2), "User2 email is not valid.");
        assert!(
            compare_ages(&user, &user2),
            "User and User2 ages are not equal."
        );
    }
}
