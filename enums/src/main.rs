fn main() {
    println!("Hello, world!");

    // 6.1. Defining an Enum
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    // let m = Message::Write(String::from("hello"));
    // m.call();
    
    // Option: return None to fix issues that can happen with null values
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    println!("some_number is: \n{:#?}", some_number);
    println!("some_char is: \n{:#?}", some_char);
    println!("absent_number is: \n{:#?}", absent_number);

    // 6.2. Match control flow construct
    let alaska = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Quarter returns: \n{:#?}", alaska);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:#?}", five);
    println!("six: {:#?}", six);
    println!("none: {:#?}", none);

    // 6.3. If let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // equivalent to
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    println!("Some(max) is: \n{:#?}", config_max);


}

// 6.3. If let

// 6.2. Match control flow construct
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Kansas,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // coin is an expression in this case
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


// Option
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// fn route(ip_kind: IpAddrKind) {}
// Inefficient method of using enums
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
