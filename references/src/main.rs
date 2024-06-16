fn main() {

    let s = String::from("Hello, World");
    let length = calculate_len(&s);
    let mut new_string = String::from("Hello");
    mutate_string(&mut new_string);
    println!("The meuated string is {new_string}");
    println!("The length of {s} is {length}")

}

fn calculate_len ( some_string : &String ) -> usize {
    some_string.len()
}

fn mutate_string (a_string: &mut String) {
    a_string.push_str(", World") 
}
