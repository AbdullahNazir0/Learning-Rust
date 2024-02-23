use std::io;
fn main() {
    let mut my_string: String = String::new();
    println!("Please Enter Your Name: ");
    // Input user name;
    io::stdin()
        .read_line(&mut my_string)
        .expect("Cannot read line.");

    // Change the name to uppercase.
    let mut res: String = String::new();
    for mut name in my_string.bytes() {
        let converted_char = if name >= 97 && name <= 122 {
            name -= 32;
            char::from(name)
        } else {
            char::from(name)
        };
        res.push(converted_char);
    }

    println!("{}", res);

    // Concatenating lowercase and uppercase.
    my_string = String::from(my_string.trim());
    let concatenated: String = my_string + &res;
    // let concatenated: String = format!("{} {}", my_string.trim(), res);
    println!("After Concatenation: {}", concatenated);

    println!("Enter a substring (in upper-case) to check: ");
    let mut sub: String = String::new();
    io::stdin().read_line(&mut sub).expect("Cannot read line.");

    let sub_arr: Vec<char> = sub.chars().collect();
    let res_arr: Vec<char> = res.chars().collect();
    for i in 0..res_arr.len() - sub_arr.len() + 1 {
        if res_arr[i] == sub_arr[0] {
            let mut k = i + 1;
            for j in 1..sub_arr.len() {
                if res_arr[k] != sub_arr[j] {
                    break;
                }
                k += 1;
            }
            println!("Sub Array found at index: {}", i);
            break;
        }
    }

    // Reversing the Uppercase string.
    let mut res2: String = String::new();
    for i in (0..res.len()).rev() {
        res2.push_str(res_arr[i] as &str);
    }
}
