/**
 * @author Dominik Dorfstetter
 * Structs in Rust are used to create custom data types (like clases)
 **/

 // simple struct (example: rgb color)
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// complex struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn name_to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

// tuple struct
struct TupleColor(u8, u8, u8);

 pub fn run() {
    // use the struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    // set individual value
    c.blue = 50;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut tc = TupleColor(255, 50, 0);

    tc.2 = 255;

    println!("Tuple Color: {} {} {}", tc.0, tc.1, tc.2);

    let mut p = Person::new("Dominik", "Dorfstetter");
    println!("Person is called {}", p.full_name());
    p.set_last_name("Secret");
    println!("Person is now called {}", p.full_name());
    println!("Person Tuple {:?}", p.name_to_tuple());
}