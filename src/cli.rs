/**
 * @author Dominik Dorfstetter
 * Access CLI Arguments
 **/
use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Dominik";
    let status = "100%";

    println!("CLI Arguments: {:?}", args);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is: {}", status);
    } else {
        println!("This is not a valid command");
    }
}