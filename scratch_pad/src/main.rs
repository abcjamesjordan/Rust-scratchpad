// folder_path = '/Users/jamesjordan/Downloads/data_Q2_2023'
// all_csv_path = 'data/all_csv_files.csv'

use polars::prelude::*;
use polars::frame::DataFrame;
use std::io::Error;
use std::fs::File;
use std::path::{Path};
use std::fs;

// use std::path;
// use std::fmt::Write;
// use polars::prelude::CsvWriter
// use polars_io::csv::CsvReader;
// use polars_io::csv::CsvWriter;
// use std::array;
// use std::env;
// use std::path::PathBuf;

fn read_data_frame_from_csv(csv_file_path: &Path,) -> DataFrame {
    CsvReader::from_path(csv_file_path)
        .expect("Cannot open file.")
        .has_header(true)
        .finish()
        .unwrap()
}

fn append_data_frame_from_df(df: &mut DataFrame, file_path: &String, headers: bool) -> Result<(), Error> {
    let mut buffer = File::create(file_path)?;
    
    CsvWriter::new(&mut buffer)
            .with_delimiter(b',')
            .has_header(headers)
            // .expect("csv written")
            .finish(df)
            .unwrap();

    Ok(())
}

fn get_all_files(folder_path: &Path) -> Vec<String>{
    // let v: Vec<String> = Vec::new();
    let mut file_names = Vec::new();
    if let Ok(entries) = fs::read_dir(folder_path) {
        // let mut file_names = Vec::new();

        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    file_names.push(file_name.to_string());
                }
            }
        }
    }
    file_names
}

// fn get_current_working_dir() -> std::io::Result<PathBuf> {
//     env::current_dir()
// }

fn main() {
    println!("Hello, world!");

    // Get file paths
    let path_cwd = Path::new(
        "/Users/jamesjordan/Documents/GitHub/rust-scratchpad/scratch_pad"
    );
    let output_file_path = path_cwd.join("data/all_csv_data.csv");
    let file_path: &Path = Path::new("/Users/jamesjordan/Downloads/data_Q2_2023/2023-05-22.csv");
    let folder_path: &Path = Path::new("/Users/jamesjordan/Downloads/data_Q2_2023/");

    print!("\npath_cwd: {:#?}\n", path_cwd);
    print!("\noutput_file_path: {:#?}\n", output_file_path);
    print!("file_path: {:#?}\n", file_path);
    print!("folder_path: {:#?}", folder_path);

    let file_names = get_all_files(folder_path);

    let mut count = 0;

    for file in file_names {
        print!("File: {:#?}\n", file);
        // assert!(!Path::new(&output_file)
        // .try_exists()
        // .expect("Can't check existence of file"));
        let path_test = output_file_path.is_file();
        let current_path = folder_path.join(file.clone()).clone();
        let current_path_string = folder_path.join(file).as_path().display().to_string();
        print!("\ncurrent path: {:#?}\n", current_path);

        let mut df: DataFrame = read_data_frame_from_csv(&current_path);
        // print!("df head: \n{:#?}", df.head(Some(5)));

        if path_test == true {
            print!("\nAppend csv\n");
            let _results = append_data_frame_from_df(&mut df, &current_path_string, path_test);
        }
        else {
            print!("\nWrite to the csv for the first time\n");
            let _results = append_data_frame_from_df(&mut df, &current_path_string, path_test);
        }
        
        count += 1;
        if count > 1 {
            break;
        }
    }

    // let path_check = Path::new("does_not_exist.txt").try_exists();
    // assert!(Path::new("/root/secret_file.txt").try_exists().is_err());


}
