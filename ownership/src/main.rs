fn main() {
   let mut s = String::from("Hello");

   s.push_str(", World");

   println!("{s}");

   { //at this point our variable is not valid
        let  s  = String::from("Hello"); //valid form this point on
        print!("{s}");
   } //s is out of scope and the memory it wa allocated is returned once it is out of scope

   {
    let s1 = String::from("Hello, World");
    let s2 = s1;
    println!("{s2}");
   }

   {
    let s1 = String::from("How are you");
    let mut s2 = s1.clone();
    s2.push_str("?");
    println!("{s2}");
   }

   let m1 = String::from("I have been moved");
   let x = 5;

   takes_ownership(m1); //value is moved since m1 is a complex type
   makes_copy(x); //makes a copy since x is a simple type

   let new_string = gives_ownership();
   let string_1 = String::from("I have been given back");
   let string_two = takes_and_gives_back(string_1);
   println!("part 1: {new_string} , part 2 : {string_two} ");
   let my_string = String::from("Return multiple values from a string");
   let (i, len) = return_multiple(my_string);
   println!("the lenth of the sentence {i} is : {len}");

}

fn takes_ownership(some_string : String){ // passing a value to a functio nworks the same as asiigining variables
    // here the value passd as an arg to this function will be moved into the varibale - some_string
    println!("{some_string}");
 } //some string goes out of scope and the memory is freed

 fn makes_copy (some_int: u32){
    println!("{some_int}");
 }


 fn gives_ownership () -> String {
        let some_string = String::from("Gives ownership");
        some_string
 }

 fn takes_and_gives_back (a_string: String) -> String {
        a_string //retrun statement should not be terminated
 }


 // rust lets us return more than one value using tuples
 fn return_multiple (s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
 }
