use std::collections::HashMap;
fn main() {
    let mut my_hashmap: HashMap<String, i32> = HashMap::new();
    my_hashmap.insert(String::from("Abdullah"), 19);
    my_hashmap.insert(String::from("Friend"), 18);
    println!("{:?}", my_hashmap);
    for (key, value) in &my_hashmap {
        println!("{}: {}", key, value);
    }
    my_hashmap.remove("Friend");
    println!("After removal: {:?}", my_hashmap);
    my_hashmap.insert("Abdullah".to_owned(), 21);
    // Set to 14 if not present, else same.
    my_hashmap.entry("Abdullah".to_owned()).or_insert(14);
    println!("After update: {:?}", my_hashmap);
    match my_hashmap.get("Abdullah") {
        Some(i) => println!("{}: {}", "Abdullah", i),
        None => println!("Not Present"),
    }
}
