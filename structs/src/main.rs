#[derive(Debug)]
struct User {
    active: bool,
    user_name: String ,
    email: String,
    sign_in_count: u64

}
struct Rectangle {
    length: u32,
    width: u32,
}
fn main() {


    let mut  user_1 : User = User {
        user_name: String::from("githuba"),
        active: true,
        email: String::from("git@gmail.com"),
        sign_in_count: 1,
    };

    user_1.email = String::from("hello@gmail.com");

    println!("{}", user_1.email);

    let rect_1 = Rectangle{length: 5, width: 8};

    println!("{:#?}", build_user(String::from("hi@gmail.com"), String::from("hello")));
    println!("{}", calculate_area(5, 8));
    println!("{}", area((5, 8)));
    println!("{}", rect_area(&rect_1))

}

fn build_user ( email: String , user_name: String) -> User {
    User {
        user_name,
        active: true,
        email,
        sign_in_count: 1,
    }
}

fn calculate_area (length: u32, width: u32  ) -> u32 {
    length * width
}


fn area (dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rect_area (plot: &Rectangle )  -> u32{
    plot.length * plot.width
}
