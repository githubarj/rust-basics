fn main() {
    
    let t = ([1; 2], [3; 4]); //[1;2] short form of creating an array of two 1s, and the other 4, 3s
    let (a , b ) = t; //destructuring the array

    println!("{}", a[0] + t.1[0] );

    println!("{}", add_one({
        let y = 1;
        y + 1
    }));

    println!("{}", add_loop([5;10]))

}


fn add_one (x: i32) -> i32 {
    x + 1
}

fn add_loop (x: [i32; 10]) -> i32 {
    
    let mut sum = 0;
    for i in x {
        sum += i
    }

    sum

}