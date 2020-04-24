/**
 * @author Dominik Dorfstetter
 * 
 * Vectors are resizeable arrays
 **/

 use std::mem;

 pub fn run() {
     // array of fixed length with 5 elements
     let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
 
     numbers[2] = 20;
 
     // add to the vector
     numbers.push(5);
     numbers.push(6);

     numbers.pop();

     println!("{:?}", numbers);
 
     println!("First value: {}", numbers[0]);
 
     // Get array length
     println!("Vector length: {}", numbers.len());
 
     // Vectors are heap allocated
     println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
 
     // Get slice of array
     let slice: &[i32] = &numbers[1..4];
     println!("{:?}", slice);

     // loop through vector, but x is immutable
     for x in numbers.iter() {
        println!("Number: {}", x);
     }

     // loop through vector, but y is mutable
     for x in numbers.iter_mut() {
        *x *= 2; // multiply value by 2
        println!("Double: {}", x);
     }
 }