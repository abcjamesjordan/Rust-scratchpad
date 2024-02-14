use std::fs::File;
// use std::io::ErrorKind;
use std::io::{self, Read};
use std::error::Error;

// fn main() {
fn main() -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> == Any type of error message
    // ----------------------------------------------------------------------------
    
    // Panic introductions
    
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];
    // ----------------------------------------------------------------------------

    // ----------------------------------------------------------------------------

    // Error handling for opening a file
    // Create the file if it doesn't exist
    // Otherwise throw an error message (permissions, or other reasons)

    // let file_name = "hello.txt";
    // let greeting_file_result = File::open(&file_name);

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     // Err(error) => panic!("Problem opening the file {}: {:?}", file_name, error),
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create(&file_name) {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file {}: {:?}", file_name, e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file {}: {:?}", file_name, other_error);
    //         }
    //     },
    // };
    // ----------------------------------------------------------------------------
    
    // Error handling using expect method

    // let file_name = "hello.txt";
    // let greeting_file = File::open(&file_name)
    //     .expect(&format!("{} should have been included in this project", file_name));
    // println!("Greeting file is: {:?}", greeting_file)
    // ----------------------------------------------------------------------------

    // Using the ? Operator
    let greeting_file = File::open("hello.txt")?; // returns an error, main() has not return type of result

    Ok(())


}

// Long method.. breaking it out in detail and not using the shortcut ? operator
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// Shortcut using the the ? operator
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Using the built in function
// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }
