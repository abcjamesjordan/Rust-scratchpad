use std::{ops::Div, vec, collections::HashMap};

fn main() {
    println!("Hello, world!");

    // ---------------------------------------- //
    println!("8.1: list of integers:");

    // Median value
    let v = vec![1, 2, 2, 2, 4, 5, 2, 320, 409324, 439328, 438, 283];
    let v_mode = vec![1, 2, 2, 2, 4, 5, 2, 320, 409324, 439328, 438, 283];
    let median_of_v = find_median_value(v);
    println!("Median of V is: {:?}", median_of_v);

    let mut v_hash_map = HashMap::new();
    for value in v_mode {
        let count = v_hash_map.entry(value).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", v_hash_map);
    let mut max_value = 0;
    let mut max_key = 0;
    for (key, value) in &v_hash_map {
        if value > &max_value {
            max_value = *value;
            max_key = *key;
        }
    }
    println!("Mode of vector is {}, and occurs {} times", max_key, max_value);

    println!("\n8.2: convert strings to pig latin:");

    let testing_word = "apple";
    println!("Testing pig latin of word {}:", testing_word);

    let testing_word_pig_latin = pig_latin(testing_word);
    println!("Pig latin results {}:", testing_word_pig_latin);

    // for value in v {
    //     v_hash_map.entry(value)
    // }


    // ---------------------------------------- //

}

fn pig_latin(word: &str) -> String {
    if word.starts_with("a") | word.starts_with("e") |
    word.starts_with("i") | word.starts_with("o") | word.starts_with("u") {
        println!("Starts with vowel");
        let pig_word = format!("{word}-hay");
        pig_word
    }
    else {
        println!("Starts with consonant");
        let first_letter = &word[0..1];
        // let first_letter = word.chars().next().unwrap();
        let word_last_off: &str = &word[1..];
        let pig_word = format!("{word_last_off}-{first_letter}ay");
        pig_word
    }
}

fn find_median_value(mut v: Vec<i32>) -> f64 {
    v.sort();
    println!("v: {:?}", v);

    let length_of_v = v.len() as i32;
    let length_of_v_mod = length_of_v % 2;

    if length_of_v_mod == 0 {
        println!("length of vector: {}", length_of_v);
        // Get the median index
        let median_index = length_of_v.div(2) as usize;
        println!("median_index value: {}", median_index);
        // Print the median value
        let high_median_value = v.get(median_index).copied().unwrap() as f64;
        let low_median_value = v.get(median_index-1).copied().unwrap() as f64;
        println!("high_median_value value: {}", high_median_value);
        println!("low_median_value value: {}", low_median_value);

        let median_value = (high_median_value - low_median_value) / 2.0 + low_median_value;
        println!("median_value: {}", median_value);
        median_value
    }
    else {
        println!("length of vector: {}", length_of_v);
        // Get the median index
        let median_index = length_of_v.div(2) as usize;
        println!("median_index value: {}", median_index);
        // Print the median value
        let median_value = v.get(median_index).copied().unwrap() as f64;
        println!("median value: {}", median_value);
        median_value
    }
}
