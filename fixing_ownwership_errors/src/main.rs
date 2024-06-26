fn main() {
    println!("Hello, world!");

    let mut nameVec = vec![String::from("Richard")];
    
    stringify_name_with_title(&mut nameVec);
    
    let first: &String =  &nameVec[0];
    println!("THe first name is : {}", first);


}


fn stringify_name_with_title(name: &mut Vec<String>) -> String {
 
    let mut full_name = name.join(" ");
    full_name.push_str("Jeremy");

    full_name   
}