use std::io;

fn main() {
    println!("Enter a number");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    // `parse` returns a Result which we need to handle. 
    // Here we handle the case where the input is not a valid integer.
    let x: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    // Use the parsed integer `x` for comparison.
    if x > 10 { //this condition must be a bool and it cannot check for if a variable is undefined like in js
        println!("The number {} is larger than 10", x);
    } else {
        println!("The number {} is smaller than or equal to 10", x);
    }

    println!("Hello, world!");

     let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    } // rust evaluates only the first true condition and does not even check the rest

    //you can use if in a let condition
    let condition : bool = true;
    let b = if condition {5} else {6};
    println!("{b}");

            let mut x = 0;

      let result =  loop {
            x+= 1;
        if x > 10 {
       
            break x + 2;
        }
    };
        println!("{result}");

          let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

      let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
     let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
      for number in( 1..4 ).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
