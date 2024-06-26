use log::debug;

#[derive(Debug)]
struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle { // implementation block
    fn rect_area(&self) -> u32 { // first parameter must be always self
        self.length * self.width
    }

    fn can_hold (&self, b: &Rectangle) -> bool { // methods have self as their first parameter
        self.rect_area() > b.rect_area()
    }

    fn create_square ( side : u32 ) -> Self { // associated functions do not take self and are often used as constructors
        Self {
            width: side,
            length: side
        }
    }
}

fn build_user(email: String, user_name: String) -> User {
    User {
        user_name,
        active: true,
        email,
        sign_in_count: 1,
    }
}

fn calculate_area(length: u32, width: u32) -> u32 {
    length * width
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn main() {
    let mut user_1: User = User {
        user_name: String::from("githuba"),
        active: true,
        email: String::from("git@gmail.com"),
        sign_in_count: 1,
    };

    user_1.email = String::from("hello@gmail.com");

    println!("{}", user_1.email);

    let rect_1 = Rectangle { length: 5, width: 8 };

    println!("{:#?}", build_user(String::from("hi@gmail.com"), String::from("hello")));
    println!("{}", calculate_area(5, 8));
    println!("{}", area((5, 8)));
    println!("the area of the rectangle :{:#?} is {}", rect_1, rect_1.rect_area());
    dbg!(&rect_1);
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        length: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("The square is {:#?}", Rectangle::create_square(5)); // we call an associated function with ::
}
