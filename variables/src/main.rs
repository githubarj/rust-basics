use std::io;
fn main () {
    let mut x = 5; // making a variable mutable as they are immutable by default
    println!("Your number is {x}");

    x = 6;

    println!("Your new number is {x}");

    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 *3 ;//a constant declaration - cannot be set to mutable and cannot be set to a value that can be computed during runtime
        // you also have to annonate the data tye of a constant
    println!("Your seconds are {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 2;

    {
        let y = y * 3;
        println!("The inner scope of y is {y}");
    }

    println!("The value of y is {y}");

    println!("Type the word whose length you want to know");

    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
    let a = a.trim().len();
    println!("The word lenght is {a}");

}