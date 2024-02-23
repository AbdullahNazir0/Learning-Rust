// #![allow(dead_code)]

// Collections that store data in heap. (More than one value)
// 1. Vectors
// 2. Strings
// 3. Hashmaps

fn main() {
    // Creating vectors.
    let mut my_vector: Vec<i32> = Vec::new();
    // Or
    let my_vector2 = vec![1, 2, 3];

    // Updating vectors.
    my_vector.push(2);

    // Reading values from vector.
    let third = &my_vector2[2]; // It will panic if the index is out of bound.
    println!("Third: {third}");
    // Or
    let third: Option<&i32> = my_vector2.get(2); // It will handle it gracefully.
    match third {
        Some(data) => println!("Third: {data}"),
        None => println!("No data at index"),
    }

    // Iterating over values in vector.
    for i in &my_vector2 {
        // For immutable
        println!("{i}");
    }

    for i in &mut my_vector {
        // For mutable.
        println!("{i}");
    }

    println!("{:?}, {:?}", my_vector, my_vector2);

    // -------------_>

    // Strings
    let mut _my_string = String::new();
    let data = "My data";
    let _my_string2 = data.to_string();
    let _my_string3 = String::from(data);
    // Updating String
    let mut _s1 = String::from("foo");
    // s.push_str("bar"); // For concating.
    // println!("{}", s);
    // Or
    let _s2 = String::from("bar");
    let _s3 = _s1 + &_s2; // Second need to be borrowed

    // For multiple concatenations, format! macro will be preferred over + operator.
    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");
    // let game = t1 + "-" + &t2 + "-" + &t3;
    // println!("{game}");
    // Instead.
    let game = format!("{t1}-{t2}-{t3}");
    println!("{game}");

    // Accessing string by index like s[0] is error in Rust.
    // The type String cannot be indexed by integer.
    // game[2];
    // println!("{game[2]}");

    // Iterating on strings.
    // If you want to get chararcters.
    for i in game.chars() {
        println!("{i}");
    }
    // If you want to get bytes.
    for i in game.bytes() {
        println!("{i}");
    }

    // ------->

    use std::collections::HashMap;

    // HashMaps
    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("Team1".to_owned(), 51);
    my_hashmap.insert(String::from("Team2"), 62);
    // All of the keys must have same type, and so does the values.
    my_hashmap.insert(String::from("Team3"), 11);
    // my_hashmap.insert(String::from("Team1"), 52);    Will be overwritten
    println!("{my_hashmap:#?}");

    // Accessing values.
    let team = "Team1".to_owned();
    let _score = my_hashmap.get(&team).copied().unwrap_or(0);
    // println!("{team}: {_score}");

    // Iterating over hashmap. (Similar to vector)
    for (key, value) in &my_hashmap {
        println!("{key}: {value}");
    }

    // Updating Hashmaps.
    my_hashmap.entry(String::from("Team1")).or_insert(71);
    // It will add team and 71 if it doesn't exist. Else it will not change.
    println!("{my_hashmap:#?}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //The or_insert method returns a mutable reference (&mut V) to the value for the specified key.
        *count += 1;
    }
    println!("{:?}", map);
}
