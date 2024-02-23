fn sum_of_even_numbers(nums: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for num in nums {
        if num % 2 == 0 {
            sum += num;
        }
    }
    sum
}

fn main() {
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original slice: {:?}", numbers);
    let result: i32 = sum_of_even_numbers(&numbers);
    println!("Sum of even numbers: {}", result);

    // Ensure the original slice is still accessible and unchanged
    println!("Original slice: {:?}", numbers);
}
