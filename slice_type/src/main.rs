fn main() {
    let mut words = String::from("Hello, world!");
    let word = first_word(&words);

    println!("{words} , {word}");

    {
        let a = String::from("Hello World");
        let a_slice1 = &a[..=5]; // to start from index 0 you do not have to include the starting index
        let  a_slice2 = &a[6..]; // to go to last index do the same
        let a2 = &a;

        println!("{a} , {a_slice1} , {a_slice2} , {a2}");

        let s = String::from("hello");

        let len = s.len();

        let slice = &s[0..len]; // to go to the end of the array
        let slice = &s[..]; // also goes to the end of the array from the beginning

    }
    {
        let tup = (7 , 8 , 6.5);
        let (x , y , z ) = tup;
        println!("{:?}", tup);
    }
    {
        let arr : [&str; 12] =  ["january", "february", "march", "april", "may", "june", "july",
        "august", "september", "october", "november", "december"];
        println!("{:?}", arr);
    }
}


fn first_word (s: &String) -> usize {

    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

