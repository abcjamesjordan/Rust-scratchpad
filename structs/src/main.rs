fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    user1.active = false;
    user1.username = String::from("anotherusername");
    user1.sign_in_count = 2;
    let user9 = build_user("testing@example.com".to_string(), "exampleusername".to_string());

    // ------- struct update syntax ------- //
    // (same outcome for user2 and user3)
    // Notes:
    // ---> in user2 example belowuser1 no longer exists if copying the string parameters as they are moved rather than copied
    //      this is because strings live on the heap rather than the stack, heaps are hard to keep track of
    //      and stacks are easy so the stackable attributes (bool, int) can be copied and persist on!!

    let _user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another2@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // OR
    let _user3 = User {
        email: String::from("another@example.com"),
        ..user9
    };

    // ------ Tuple structs ------ //
    // Useful for minimizing redundant structs
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Unit-like structs without fields???
    let _subject = AlwaysEqual;

    
    // ------ 5.2 Example ------- //
    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );

    // println!("rect1 is: \n{:?}", rect1); // print structs requires #[derive(Debug)] above struct declaration
    // println!("rect1 is: \n{:#?}", rect1); // pretty print

    // dbg! --> outputs to standard error stderr instead of standard output stdout (println! uses this)
    // let scale = 2;
    // let rect2 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    // dbg!(&rect2);

    // ------ Methods ------ //
    let rect0 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect0.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("Square is {:#?}", sq);
}

// ------ 5.3 Methods ------ //
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// ------ 5.2 Example ------ //
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }



// ------ 5.1 ------ //

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
