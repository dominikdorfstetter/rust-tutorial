/**
 * @author Dominik Dorfstetter
 **/

 pub fn run() {
    let person: (&str, &str, i8) = ("Dominik", "Adesso Austria GmbH", 30);

    println!("{} works for {} and is {}", person.0, person.1, person.2);
}