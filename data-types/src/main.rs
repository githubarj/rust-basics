fn main() {
    let x : i8 = 56;
    println!("Your number is {x}");
    let y = 2.1;
    println!("My floating point number is {y}"); 
        // addition
 

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;
     println!("My quotient is {quotient}"); 
      println!("My truncated  is {truncated}");  
      println!("My remainder is {remainder}"); 

      let f = true;
      let t : bool = false;
      let m = 'Z'; //we write characters with single quotes
      let g : char = 'T';

      let tup : (i8, u32 , f32)  = (-12, 456 , 2.5);
      // we can use patern matching to destructure a tuple
      let (q, w, e) = tup;
      let dec = tup.2;
      println!("{dec}");

      // arrays in rust have a fixed length  - store memory in stack instead of heap
      // vectors provided by the std lib are allowed to grow or shrink in size
       let a = [1, 2, 3, 4, 5];
       // declaring an array type - type of elements in array followed by lenght of array
       let mk : [i8; 5] = [1, 2, 3, 4, 5];
       // you can also initialize an array to have same value by declaring the value followed by legth
       let sv = [4;5];
}
