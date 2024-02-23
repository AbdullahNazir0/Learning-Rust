use std::mem;

fn increment_array(array: [i32; 5]) -> [i32; 5] {
    let mut new_array: [i32; 5] = [0; 5];
    new_array[0] = array[0] + 1;
    new_array[1] = array[1] + 1;
    new_array[2] = array[2] + 1;
    new_array[3] = array[3] + 1;
    new_array[4] = array[4] + 1;

    new_array
}

fn analyze_slice(slice: &[i32]) {
    println!("The first element of slice is {}", slice[0]);
    println!("The total elements of slice are {}", slice.len());
}

fn main() {
    // Arrays.
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let my_array2: [i32; 5] = [0; 5];
    println!("{:?}", my_array);
    println!("{:?}", my_array2);
    println!("First Element: {:?}", my_array[0]);
    println!("Array length is {}", my_array.len());
    println!("Array occupies {}", mem::size_of_val(&my_array));
    println!("Incremented Array is {:#?}", increment_array(my_array));

    // Slices.
    // It will borrow full array.
    analyze_slice(&my_array);
    // It will borrow a part of array.
    analyze_slice(&my_array[1..4]);

    // Compare empty array and slice
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);
}
