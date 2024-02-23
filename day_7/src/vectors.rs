fn main() {
    let mut my_vector: Vec<i32> = Vec::new();

    // my_vector.push(1);
    // my_vector.push(2);
    // my_vector.push(3);
    // my_vector.push(4);
    // my_vector.push(5);
    // my_vector.push(6);
    // my_vector.push(7);
    // my_vector.push(8);
    // my_vector.push(9);
    // my_vector.push(10);

    for i in 1..11 {
        my_vector.push(i);
    }

    println!("{my_vector:?}");

    let mut index = 0;
    for i in &my_vector {
        println!("Index: {index}, Value: {i}");
        index += 1;
    }

    my_vector.pop();
    println!("Vector after pop(): {my_vector:?}");

    my_vector.remove(5);
    println!("Vector after remove(5): {my_vector:?}");
}
