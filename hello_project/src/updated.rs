use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn get_header(file_path: &str) -> String {
    println!("Getting header for file: {}", file_path);
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
    let header_length = header.split('|').count();
    let f = File::open(input_path).unwrap();
    let f = BufReader::new(f);

    let temp_file = File::create(output_path)?;
    let writer = Arc::new(Mutex::new(BufWriter::new(temp_file)));

    let lines: Vec<String> = f.lines().flatten().collect();
    let non_ascii_line_numbers = Arc::new(Mutex::new(Vec::new()));
    let non_matching_fields_line_numbers = Arc::new(Mutex::new(Vec::new()));
    let empty_lines_line_numbers = Arc::new(Mutex::new(Vec::new()));

    lines.par_iter().enumerate().for_each(|(index, line)| {
        let mut line = line.clone();
        if !line.is_empty() {
            line = line.trim().to_string();
            if !line.chars().all(|c| c.is_ascii()) {
                line.retain(|c| c.is_ascii());
                non_ascii_line_numbers.lock().unwrap().push(index + 1);
            }

            let line_length = line.split('|').count();
            if header_length != line_length {
                non_matching_fields_line_numbers.lock().unwrap().push(index + 1);
            } else {
                // Write the corrected lines to the file
                let mut writer = writer.lock().unwrap();
                writeln!(writer, "{}", line).unwrap();
            }
        } else {
            empty_lines_line_numbers.lock().unwrap().push(index + 1);
        }
    });

    // Print out the lines with issues
    let non_ascii_lines = Arc::try_unwrap(non_ascii_line_numbers).unwrap().into_inner().unwrap();
    let non_matching_fields_lines = Arc::try_unwrap(non_matching_fields_line_numbers).unwrap().into_inner().unwrap();
    let empty_lines_lines = Arc::try_unwrap(empty_lines_line_numbers).unwrap().into_inner().unwrap();

    let mut combined: Vec<(usize, &str)> = non_ascii_lines.iter()
        .map(|&x| (x, "non-ascii characters removed"))
        .chain(non_matching_fields_lines.iter().map(|&x| (x, "number of fields did not match header")))
        .chain(empty_lines_lines.iter().map(|&x| (x, "empty line skipped")))
        .collect();

    combined.sort();

    for (val, id) in combined {
        println!("Line {}: {}", val, id);
    }

    Ok(())
}


fn main() {
    //  -> std::io::Result<()>
    // Ok(())

    // Declare constants
    let path = "/Users/jj013794/Documents/Github/raw-file-processing-poc/data/small_data_file.txt";
    let output_path = "/Users/jj013794/Documents/Github/raw-file-processing-poc/data/output_20240611.txt";
    // let header = "Member_ID|First_Name|Last_Name|Middle_Initial|Sub_ID|Sub_Party_ID|DOB|Gender|Relationship|Street_Addr_Line_1|Street_Addr_Line_2|City|State|Zip_Code|Plan_Code|Effective_Date|Term_Date|Phone_Num|Email|Product_Name|Corp_Code|Emplyer_Acct_ID|Group_Number|Employer_Account_Name|Mbr_Nbr|Section_Name|Section_Number|Funding_Type|Sub_SSN|Loc_Cd|Loc_Desc|region|district|network_prefix|benefit_plan_id|quality_incentive";
    let header = get_header(path);

    // Start a timer
    let start = Instant::now();

    // ~ 24 seconds full file
    let _ = basic_processing(&path, &header, &output_path);

    // ~ 9 seconds full file
    let _ = parallel_processing(&path, &header, &output_path);

    // Stop the timer and print results
    let duration = start.elapsed();
    println!("Basic processing completed in {:?}.", duration);
}
