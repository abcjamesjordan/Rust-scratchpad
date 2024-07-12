// This project aims to open a file and do some basic operations on it.
// Goals for the operations include:
// 1. Print/remove non-ascii characters
// 2. Print/remove lines with bad number of columns
// 3. Uppercase a column

use std::fs::File;
// use std::io::{Read, Write}; // used for all at once retain of characters
use std::io::{BufRead, BufReader, Write, BufWriter};
use std::time::Instant;
use rayon::prelude::*; // used for paralell processing
use std::sync::{Arc, Mutex}; // used for memory locking

fn strip_with_annotation (input_file_path: &str, output_file_path: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let file = File::open(input_file_path)?;
    let reader = BufReader::new(file);

    let temp_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(temp_file);

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(mut line) => {
                if !line.chars().all(|c| c.is_ascii()) {
                    println!("Removed non-ASCII characters from line {}", index+1);
                    line.retain(|c| c.is_ascii());
                    writeln!(writer, "{}", line)?;
                } else {
                    writeln!(writer, "{}", line)?;
                }
            },
            Err(e) => {
                eprintln!("Failed to read line {}: {}", index+1, e);
            }
        }
    }

    let duration = start.elapsed();
    println!("Non-ASCII processing with basic techniques completed in {:?}.", duration);

    Ok(())
}

fn strip_with_parallel_processing (input_file_path: &str, output_file_path: &str) -> std::io::Result<()> {
    let start = Instant::now();

    let file = File::open(input_file_path)?;
    let reader = BufReader::new(file);

    let temp_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(temp_file);

    // Read the file into a Vec<String>
    let mut lines: Vec<String> = reader.lines()
        .filter_map(|line| line.ok())
        .collect();

    // Process each line in parallel
    lines.par_iter_mut().for_each(|line| {
        if !line.chars().all(|c| c.is_ascii()) {
            line.retain(|c| c.is_ascii());
        }
    });

    // Write the processed lines to the output file
    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    let duration = start.elapsed();
    println!("Non-ASCII processing with parallel processing completed in {:?}.", duration);

    Ok(())
}

fn strip_with_parallel_annotation(input_file_path: &str, output_file_path: &str) -> std::io::Result<()> {
    // Start a timer
    let start = Instant::now();

    // Open the input file to a variable
    let file = File::open(input_file_path).map_err(|e| {
        eprintln!("Failed to open input file: {}", e);
        e
    })?;

    // Create a buffer reader from the file
    let reader = BufReader::new(file);

    // Create a new file for the output
    let temp_file = File::create(output_file_path).map_err(|e| {
        eprintln!("Failed to create output file: {}", e);
        e
    })?;

    // Create a buffer writer for the output file
    let mut writer = BufWriter::new(temp_file);

    // Read the input file into a loopable vector of strings
    let mut lines: Vec<String> = reader.lines()
        .filter_map(|line| line.ok())
        .collect();

    // Create a thread-safe vector to store line numbers with issues
    let line_numbers = Arc::new(Mutex::new(Vec::new()));

    // Process each line in parallel
    lines.par_iter_mut().enumerate().for_each(|(index, line)| {
        // Check if the line contains non-ASCII characters
        if !line.chars().all(|c| c.is_ascii()) {
            // Remove the non-ASCII characters
            line.retain(|c| c.is_ascii());

            // Add the line number to the vector
            let mut line_numbers = line_numbers.lock().unwrap();
            line_numbers.push(index + 1);
        }
    });

    // Write the processed lines to the output file
    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    // Print the line numbers after all lines have been processed
    let mut line_numbers = Arc::try_unwrap(line_numbers).unwrap().into_inner().unwrap();
    line_numbers.sort();
    for line_number in line_numbers {
        println!("Removed non-ASCII characters from line {}", line_number);
    }

    // Calculate the duration of the process
    let duration = start.elapsed();
    println!("Non-ASCII processing with parallel processing annotation completed in {:?}.", duration);

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Hello, project!");

    let input_file_path = "data_file.txt";
    let output_file_path = "clean_data_file.txt";

    // strip_with_annotation(&input_file_path, &output_file_path)?;

    // strip_with_parallel_processing(&input_file_path, &output_file_path)?;

    strip_with_parallel_annotation(&input_file_path, &output_file_path)?;

    Ok(())
}

// fn strip_without_annotation (file_path: &str) -> std::io::Result<()> {
//     // open the file in read-only mode
//     let mut file = File::open(file_path)?;

//     // read file contents into a string
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     // retain only ASCII characters
//     contents.retain(|c| c.is_ascii());

//     // open the file in write mode
//     // let mut file = File::create(&file_path)?;
//     let mut clean_file = File::create("clean.txt")?;

//     // write the cleaned file to the same file
//     clean_file.write_all(contents.as_bytes())?;

//     println!("Successfully removed non-ASCII characters from the file.");

//     Ok(())
// }
