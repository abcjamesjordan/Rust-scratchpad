use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // Stores data on the heap!
    // Homogeneous:
    // all keys must have same type and all values must have same type

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:#?}", scores);
    println!("Blue: {:#?}", scores.get("Blue").copied().unwrap_or(0));
    println!("Yellow: {:#?}", scores.get("Yellow").copied().unwrap_or(0));
    println!("Red: {:#?}", scores.get("Red").copied().unwrap_or(0));
    // .unwrap_or(0): returns 0 if nothing (None) is returned, otherwise returns value
    println!("Blue: {:#?}", scores.get("Blue")); // returns Some(10) as Option<&i32>
    println!("Blue: {:#?}", scores.get("Blue").copied()); // returns Some(10), but as Option<i32>

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Remember to keep in mind ownership
    // Values are moved to new hash-map and not valid after move

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("field_name: {}", field_name); // errors out if left uncommented
    // println!("field_value: {}", field_name); // errors out if left uncommented

    // Hash map values will be overwritten if added twice

    // Entry takes key as parameter to check if exists
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(500);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
