fn main() {
    println!("Hello, world!");
    another_function(5, 'h' );
    my_func(5, 5);
    let x = five();
    println!("{x}");
    let ati = add_to_one(5);
    println!("{ati}");
}

fn another_function (x : i32, unit_char: char) {
    println!("Another function {x}{unit_char}");
}

fn my_func(y: i32, b: i32) {
    let result = {{
        let m = 5;
        (y + b) * m
    }};

    println!("{result}")
}

fn five () -> i32 {
    5
}

fn add_to_one(x: i32) -> i32 {
    x + 1
}