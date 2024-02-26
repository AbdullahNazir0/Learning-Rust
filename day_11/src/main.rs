// Closures and Iterators.

use std::collections::HashMap;

mod tasks;

fn main() {
    // Closures.
    // let sum_closure = |num1, num2| num1 + num2;
    // println!("{}", sum_closure(2, 3));

    // let x = 10;
    // let capturing_closure = || println!("{}", x);
    // capturing_closure();

    // let mut counter_value = 1;
    // let mut counter_closure = || {
    // counter_value += 1;
    // println!("Value after increment: {}", counter_value);
    // };
    // for _i in 0..5 {
    //     counter_closure();
    // }

    // let generic_closure = |val1, val2| {
    //     if val1 > val2 {
    //         val1
    //     } else{
    //         val2
    //     }
    // };
    // println!("{}", generic_closure(2, 3));

    // let x = String::from("Hello");
    // let mut y = String::new();
    // let owned_closure = move|| y = x;
    // owned_closure();
    // println!("x:{}, y:{}", x, y); // error

    // Iterators.
    let v1 = vec![1, 2, 3];
    let _v1_iter = v1.iter();
    // for i in &v1 {
    //     println!("{}", i);
    // }
    // for i in _v1_iter {
    //     println!("{}", i);
    // }
    // for i in v1.iter() {
    //     println!("{}", i);
    // }
    // sum(), next() -> consuming adaptors
    // Consuming iterators are iterators that consume the elements they iterate over. Once an element is produced, it cannot be used again by the same iterator.
    // map(), filter() -> iterator adaptors
    // Iterator adaptors are methods provided by the Iterator trait that transform an existing iterator into a new iterator. These methods do not consume the original iterator but instead produce a new iterator with modified behavior.

    // Tasks.
    // 1.
    let sample_vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    let new_sample_vector = tasks::filter_numbers(&sample_vector, |num| num % 2 == 0);

    println!("{:?}", new_sample_vector);

    // 2.
    let result = tasks::calculate_statistics(sample_vector.iter());
    println!("{:?}", result);

    // 3.
    let strings_1 = vec![String::from("Abdullah"), String::from("Umer")];
    let res1 = tasks::transform_strings(&strings_1, |s| s.to_uppercase());
    println!("{:?}", strings_1);
    println!("{:?}", res1);

    // 4.
    //     Chaining Iterators:
    // Explore the chaining capabilities of iterators using methods like filter, map, collect, and take_while.
    // Create an iterator that filters even numbers, squares the remaining numbers, and collects them into a vector.
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<i32> = vector
        .iter()
        .filter(|n| *n % 2 == 0)
        .map(|n2| n2 * n2)
        .collect();
    println!("{:?}", vector);
    println!("{:?}", result);

    // 5.
    let mut _fibonacci = tasks::Fibonacci {
        current: 0,
        next: 1,
    };

    for num in _fibonacci.take(10) {
        println!("{}", num);
    }

    // 6.
    let string: String =
        String::from("this is a sentence with repitetive words. sentence is repeated.");
    let result2 = tasks::get_sorted_unique_words(&string, |s1| {
        let mut my_hashmap: HashMap<String, usize> = HashMap::new();
        for i in s1.split_whitespace() {
            if my_hashmap.contains_key(i) {
                my_hashmap.insert(i.to_string(), 2);
            } else {
                my_hashmap.insert(i.to_string(), 1);
            }
        }
        my_hashmap
    });
    println!("{}", string);
    println!("{:?}", result2);
}
