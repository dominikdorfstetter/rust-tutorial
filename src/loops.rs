/**
 * @author Dominik Dorfstetter
 * Infinite Loop, While Loop, For Range
 **/

 pub fn run() {
    let mut count: u8 = 0;

    // infinite loop - produces 'attempt to add with overflow'
    loop {
        count += 1;
        println!("Number >> {}", count);
        if count == 20 { break };
    }

    // while loop 'fizzbuzz'
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz >> {}", count);
        } else if count % 3 == 0 {
            println!("fizz >> {}", count);
        } else if count % 5 == 0 {
            println!("buzz >> {}", count);
        } else {
            println!("not dividable by 15, 3 or 5 >> {}", count);
        }

        count += 1;
    }

    // for range 'fizzbuzz'
    for x in 100..200 {
        if x % 15 == 0 {
            println!("fizzbuzz >> {}", x);
        } else if x % 3 == 0 {
            println!("fizz >> {}", x);
        } else if x % 5 == 0 {
            println!("buzz >> {}", x);
        } else {
            println!("not dividable by 15, 3 or 5 >> {}", x);
        }
    }

 }