/**
 * @author Dominik Dorfstetter
 * 
 * Functions 
 **/
pub fn run() {
    say_hello("Holà", "Roberta");
    let get_sum = add(12, 23);
    println!("The sum of 12 and 23 is {}", get_sum);

    // Closure Function
    // can use outside variables in the same scope
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C-Sum {}", add_nums(3, 3));
}

// void that takes 2 Strings and prints them out formatted, no return value
fn say_hello(greeting: &str, name: &str) {
    println!("{} {}, nice to meet you!", greeting, name);
}

// adds 2 numbers & returns the solution
// no return keyword needed only one line
fn add(n1: i32, n2: i32) -> i32 { n1 + n2 }

