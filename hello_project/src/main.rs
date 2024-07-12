#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn get_header(file_path: &str) -> String {
    if !std::path::Path::new(file_path).exists() {
        eprintln!("Error: File not found: {}", file_path);
        std::process::exit(1);
    } else {
        println!("Getting header for file: {}", file_path);
    }
    let f = File::open(file_path).unwrap();
    let f = BufReader::new(f);
    let mut header = String::new();

    for mut line in f.lines().flatten() {
        line.trim().to_string().retain(|c| c.is_ascii());
        line.trim().to_string();
            if !line.chars().all(|c| c.is_ascii()) {
                line.retain(|c| c.is_ascii());
            }
        header = line;
        break;
    }
    header
}


fn basic_processing (input_path: &str, header: &str, output_path: &str) -> std::io::Result<()> {

    let header_length = header.split('|').count();
    let f = File::open(input_path).unwrap();
    let f = BufReader::new(f);

    let temp_file = File::create(output_path)?;
    let mut writer = BufWriter::new(temp_file);

    for (index, mut line) in f.lines().flatten().enumerate() {
        if !line.is_empty() {
            line.trim().to_string();
            if !line.chars().all(|c| c.is_ascii()) {
                line.retain(|c| c.is_ascii());
                println!("Line {}: non-ascii characters removed", index+1);
            }
            else {
                // ASCII test passed
            }
        }
        else {
            println!("Line {}: empty line skipped", index+1);
            continue;
        }

        let line_length = line.split('|').count();
        if header_length != line_length {
            println!("Line {}: number of fields did not match header", index+1);
            continue;
        }
        else {
            // header length test passed, write to file
            writeln!(writer, "{}", line.trim())?;
        }
    }

    Ok(())
}

fn parallel_processing(input_path: &str, header: &str, output_path: &str) -> std::io::Result<()> {
    // Count the fields supplied in the header
    let header_length = header.split('|').count();
    
    // Open the input file to a variable
    let f = File::open(input_path).unwrap();
    // Create a buffer reader from the file
    let f = BufReader::new(f);

    // Create a new file for the output
    let temp_file = File::create(output_path)?;
    // Create a buffer writer from the output file
    let writer = Arc::new(Mutex::new(BufWriter::new(temp_file)));

    // Read the input file into a vector of strings
    let lines: Vec<String> = f.lines().flatten().collect();

    // Create a thread-safe vector to store line numbers with issues
    let non_ascii_line_numbers = Arc::new(Mutex::new(Vec::new()));
    let non_matching_fields_line_numbers = Arc::new(Mutex::new(Vec::new()));
    let empty_lines_line_numbers = Arc::new(Mutex::new(Vec::new()));

    // Process each line in parallel
    lines.par_iter().enumerate().for_each(|(index, line)| {
        if !line.is_empty() {
            let mut line = line.clone();
            // Trim the string of leading and trailing whitespace
            line = line.trim().to_string();
            // Check if the line contains non-ASCII characters
            if !line.chars().all(|c| c.is_ascii()) {
                // Remove the non-ASCII characters
                line.retain(|c| c.is_ascii());
                // Add the line number to the non-ASCII vector
                non_ascii_line_numbers.lock().unwrap().push(index + 1);
            }

            // If the line does not match the header length, add the line number to the non-matching fields vector
            let line_length = line.split('|').count();
            if header_length != line_length {
                non_matching_fields_line_numbers.lock().unwrap().push(index + 1);
            } else {
                // Write the corrected lines to the file
                let mut writer = writer.lock().unwrap();
                writeln!(writer, "{}", line).unwrap();
            }
        } else {
            // If the line is empty, add the line number to the empty lines vector
            empty_lines_line_numbers.lock().unwrap().push(index + 1);
        }
    });

    // Print out the lines with issues
    let non_ascii_lines = Arc::try_unwrap(non_ascii_line_numbers).unwrap().into_inner().unwrap();
    let non_matching_fields_lines = Arc::try_unwrap(non_matching_fields_line_numbers).unwrap().into_inner().unwrap();
    let empty_lines_lines = Arc::try_unwrap(empty_lines_line_numbers).unwrap().into_inner().unwrap();

    // Combine the line numbers with the issues type and sort them in ascending order
    let mut combined: Vec<(usize, &str)> = non_ascii_lines.iter()
        .map(|&x| (x, "non-ascii characters removed"))
        .chain(non_matching_fields_lines.iter().map(|&x| (x, "number of fields did not match header")))
        .chain(empty_lines_lines.iter().map(|&x| (x, "empty line skipped")))
        .collect();
    combined.sort();

    // Print out the lines with issues
    for (val, id) in combined {
        println!("Line {}: {}", val, id);
    }

    // Return success
    Ok(())
}


fn main() {
    //  -> std::io::Result<()>
    // Ok(())

    // Declare constants
    let path = "data_file.txt";
    let output_path = "clean_data_file.txt";
    let header = get_header(path);

    // Start a timer
    let start = Instant::now();

    // ~ 24 seconds full file
    // let _ = basic_processing(&path, &header, &output_path);

    // ~ 9 seconds full file
    let _ = parallel_processing(&path, &header, &output_path);

    // Stop the timer and print results
    let duration = start.elapsed();
    println!("Basic processing completed in {:?}.", duration);
}
