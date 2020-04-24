/**
 * @author Dominik Dorfstetter
 **/
pub fn run() {
 let name = "Dominik";

 // Variables are immutable by design (this means once initialized you cannot overwrite the value)
 // If you need to reassign the variable later on you need to add the keyword mut
 let mut age = 30;

 println!("My name is {} and I am {}", name, age);

 // If you reassign a variable before using it or as an example print it out
 // you will get a warning message
 age = 21;

 println!("My name is {} and I am {}", name, age);

 // Because variables are immutable at default you won't use const that often
 const APP_ID: i32 = 001203;
 println!("The app id is {}", APP_ID);

 // Assign multiple vars at once
 let ( game, mood ) = ( "Hearthstone", "great" );
 println!("{} is {}", game, mood);
}