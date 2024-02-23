use std::collections::HashMap;

fn to_uppercase(my_str: &String) -> String {
    let mut res = String::new();
    for i in my_str.chars() {
        if i >= 'a' && i <= 'z' {
            let temp = i as u8 - 32;
            res.push_str(&String::from(temp as char));
        } else {
            res.push_str(&String::from(i));
        }
    }
    res
}

fn main() {
    let mut names: Vec<String> = vec!["John".to_owned(), "Michael".to_owned(), "Tommy".to_owned()];
    let mut upper_names: Vec<String> = vec![];
    for i in &names {
        upper_names.push(to_uppercase(&i));
    }
    println!("{:?}", names);
    println!("{:?}", upper_names);

    // ------------------------------------>

    let text = "This is the sample text. This is for tesing only.";
    let text2 = to_uppercase(&String::from(text));
    let mut hashmap = HashMap::new();
    for i in text2.split_whitespace() {
        let count = hashmap.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", hashmap);
}
