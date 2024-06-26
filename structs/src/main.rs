#[derive(Debug)]
struct User {
    active: bool,
    user_name: String ,
    email: String,
    sign_in_count: u64

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

    println!("{:?}", build_user(String::from("hi@gmail.com"), String::from("hello")))

}

fn build_user ( email: String , user_name: String) -> User {
    User {
        user_name,
        active: true,
        email,
        sign_in_count: 1,
    }
}