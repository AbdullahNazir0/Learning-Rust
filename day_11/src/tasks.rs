// 1.
//Filtering Numbers:
//      Write a function that takes a vector of integers and a closure as arguments.
//      The closure should take one integer as input and return a boolean indicating whether the number should be included in the filtered vector.
//      Use this function to filter out even numbers from the vector.

pub fn filter_numbers<F>(numbers: &[i32], check_closure: F) -> Vec<i32>
where
    F: Fn(i32) -> bool,
{
    let mut new_vector = Vec::new();
    for i in numbers.iter() {
        if check_closure(*i) {
            new_vector.push(*i);
        }
    }
    new_vector
}

// 2.
// Calculating Statistics:
//      Create a function that takes an iterator over numbers and returns a struct containing the sum, average, minimum, and maximum values.
//      Use this function on a vector or range of numbers.

#[derive(Debug)]
pub struct Result {
    _sum: i32,
    _average: f64,
    _minimum: i32,
    _maximum: i32,
}

pub fn calculate_statistics<'a>(my_iterator: impl Iterator<Item = &'a i32>) -> Result {
    let mut s = 0;
    let mut count = 0;
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for i in my_iterator {
        count += 1;
        s += i;
        if min > *i {
            min = *i;
        }
        if max < *i {
            max = *i;
        }
    }
    Result {
        _sum: s,
        _average: s as f64 / count as f64,
        _minimum: min,
        _maximum: max,
    }
}

// 3.
// Mapping Strings:
//      Implement a function that takes a vector of strings and a closure.
//      The closure should take one string and apply a transformation (e.g., uppercase, lowercase, reverse) and return the modified string.
//      Use this function to transform all strings in the vector to uppercase.

pub fn transform_strings<F>(strs: &[String], my_closure: F) -> Vec<String>
where
    F: Fn(String) -> String,
{
    let mut new_vec = Vec::new();
    for i in strs.iter() {
        new_vec.push(my_closure(i.to_string()));
    }
    new_vec
}

// 5.
// Implementing Custom Iterators:
//      Define a struct that implements the Iterator trait.
//      Its next method should yield elements from a predefined sequence (e.g., Fibonacci numbers, prime numbers).

#[derive(Debug)]
pub struct Fibonacci {
    pub current: u64,
    pub next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;
        Some(self.current)
    }
}

// 6.
// Building a Simple Text Analyzer:
//      Create a function that takes a string and returns a vector of unique words, sorted alphabetically.
//      Use iterators and closures to achieve this functionality.

use std::collections::HashMap;

pub fn get_sorted_unique_words<F1>(my_str: &String, found_unique: F1) -> Vec<String>
where
    F1: Fn(&String) -> HashMap<String, usize>,
{
    let mut vec_res: Vec<String> = Vec::new();
    let results = found_unique(my_str);
    for (key, value) in results {
        if value == 1 {
            vec_res.push(key);
        }
    }
    vec_res.sort();
    vec_res
}
