/**
 * @author Dominik Dorfstetter
 * Video paused at 1:00:00
 **/

 pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // if condition
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What do you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you need to leave!");
    } else {
        println!("I need to see your ID");
    }

    // Shorthand if
    let is_of_age = if age > 21 { true } else { false };

    println!("Is of age? {}", is_of_age);
 }