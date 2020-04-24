/**
 * Strings
 * 
 * --- 2 types of strings in Rust ---
 * Immutable fixed-lenght string somewhere in memory
 * Growable, heap-allocated data structure # use when you need to modify your own string data
 * 
 * 
 * @author Dominik Dorfstetter
 * */

pub fn run() {
 // Immutable fixed length
 let fixed = "fixed";
 // dynamic, heap-allocated string
 let mut dynamic = String::from("dynamic");

 // length of string
 println!("Length of dynamic {}", dynamic.len());

 dynamic.push('X');
 println!("{} lenght {}", dynamic, dynamic.len());

 // print 
 println!("{:?}", (fixed, dynamic));

}