/**
 * @author Dominik Dorfstetter
 **/
pub fn run() {
    // print to console
    println!("Hello from print.rs");

    // --- BASIC Formatting --
    // print out number, arguments are empty curly brackets
    println!("The number is {}", 1);

    // also works with multiple arguments
    println!("{} is  a {}", "Dominik", "coder");

    // --- POSITIONAL ARGUMENTS ---
    // By number
    println!(
        "{0} is from {1}. {0} likes {2} and {3}.", 
        "Dominik", "Vienna", "Hearthstone", "Gin"
    );

    // By naming them
    println!(
        "{name} is born in {city}. {name} parents are {dad} and {mom}.",
        name = "Dominik",
        city = "Wr. Neustadt",
        dad = "Erich",
        mom = "Monika"
    );

    // Number conversion on print
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Debug trait
    println!("{:?}", (12, true, "Hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}