/**
 * @author Dominik Dorfstetter
 * 
 * Arrays are a fixed list where elements are the same data type
 **/

use std::mem;

pub fn run() {
    // array of fixed length with 5 elements
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[2] = 20;

    println!("{:?}", numbers);

    println!("First value: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice of array
    let slice: &[i32] = &numbers[1..4];

    println!("{:?}", slice);
}