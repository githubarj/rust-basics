fn main() {

    let s = String::from("Hello, World");
    let length = calculate_len(&s);
    let mut new_string = String::from("Hello");
    mutate_string(&mut new_string);
    println!("The meuated string is {new_string}");
    println!("The length of {s} is {length}");
    mutate_vec();
    mutable_reference();
    ques_three()

}

fn calculate_len ( some_string : &String ) -> usize {
    some_string.len()
}

fn mutate_string (a_string: &mut String) {
    a_string.push_str(", World") 
}

fn mutate_vec (){

    let mut v: Vec<i32> = vec![1 ,2, 3];
    let x: &i32 = &v[2];
    println!("{x}");

    v.push(5);
    println!("The final vector is {:?}", &v);
}

fn mutable_reference (){
    let mut v : Vec<i32> = vec![1, 5, 8];
    let b : &mut i32 = &mut v[2];
    *b += 5;
    println!("The new vector is {:?}", &v)

}

fn ques_three (){
    let mut v : Vec<String> = vec![String::from("A"), String::from("B")];
    let first = get_first(&v);
    if first.len() != 0 {
        v.push(String::from("C"))
    };
    println!("{:?}", v)

}

fn get_first (b : &Vec<String> ) -> &str {
    &b[0]
}