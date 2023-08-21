use std::io;

fn convert_to_farenheit(user_in: f64) -> f64 {
    let farenheit_to_return  = (5.0/9.0)*(user_in-32.0);
    farenheit_to_return 
}

fn convert_to_celcius(user_in: f64) -> f64 {
    let celcius_to_return = (user_in * (9.0/5.0)) + 32.0;
    celcius_to_return 
}

fn main() {
    println!("Celcius or degrees to convert?");
    println!("C or F?");

    loop {
        let mut user_in = String::new();

        io::stdin()
        .read_line(&mut user_in)
        .expect("Failed to read input");
        
        let user_in = user_in.trim();

        println!("Please input the value to convert.");
        let mut to_convert_num = String::new();
        io::stdin()
            .read_line(&mut to_convert_num)
            .expect("Failed to read line");
        let to_convert_num: f64 = match to_convert_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You want to convert: {to_convert_num}");

        if user_in == "C" {
            println!("You want {user_in}");
            let num_converted = convert_to_farenheit(to_convert_num);
            println!("{to_convert_num} converted is {num_converted}");
            break;
        }
        else if user_in == "F" {
            println!("You want {user_in}");
            let num_converted = convert_to_celcius(to_convert_num);
            println!("{to_convert_num} converted is {num_converted}");
            break;
        }
        else {
            println!("Please enter C or F");
            continue;
        }
    }
    
    // C = 5/9 (F-32)

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");

    // for number in (1..=3).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");
}