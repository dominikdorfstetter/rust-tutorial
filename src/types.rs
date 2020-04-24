/* *
 * @author Dominik Dorfstetter
 * --- Primitive types of Rust ---
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
 * Floats: f32, f64
 * Boolean: (bool)
 * Characters: (char)
 * Tuples
 * Arrays (are fixed length by default, for dynamic arrays use Vectors)
 * 
 * --- Introduction ---
 * Rust is a statically typed language, which means that it must know the types
 * of all the variables at compile time. However, the compiler can usually infer
 * what type we want to use based on the value and how we use it.
 * 
 * 
 * */
pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 92392392939239239;

    // Boolean
    let is_active: bool = true;

    // Boolean by expression
    let is_greater: bool = 10 < 5;

    // Characters, single quotation marks only
    let a: char = 'a';
    let devil: char = '😈';

    println!("{:?}", (x, y, z, is_active, is_greater, a, devil));

    // Print out max-length
    println!("Maximum for i32: {}", std::i32::MAX);
    println!("Maximum for u64: {}", std::u64::MAX);
    println!("Maximum for f32: {}", std::f32::MAX);
}