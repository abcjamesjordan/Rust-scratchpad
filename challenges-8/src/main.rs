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

    // for value in v {
    //     v_hash_map.entry(value)
    // }


    // ---------------------------------------- //

}
// #[derive(Debug)]
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
