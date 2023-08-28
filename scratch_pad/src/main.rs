// folder_path = '/Users/jamesjordan/Downloads/data_Q2_2023'
// all_csv_path = 'data/all_csv_files.csv'
// use polars::prelude::*;
// use polars::frame::DataFrame;
// use std::array;
use std::path::Path;
use std::fs;

// fn read_data_frame_from_csv(csv_file_path: &Path,) -> DataFrame {
//     CsvReader::from_path(csv_file_path)
//         .expect("Cannot open file.")
//         .has_header(true)
//         .finish()
//         .unwrap()
// }

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

fn main() {
    println!("Hello, world!");

    let file_path: &Path = Path::new("/Users/jamesjordan/Downloads/data_Q2_2023/2023-05-22.csv");
    let folder_path: &Path = Path::new("/Users/jamesjordan/Downloads/data_Q2_2023/");
    print!("file_path: {:#?}\n", file_path);
    print!("folder_path: {:#?}", folder_path);

    let file_names = get_all_files(folder_path);

    let mut count = 0;

    for file in file_names {
        print!("File: {:#?}\n", file);
        count += 1;
        if count > 5 {
            break;
        }
    }

    
    // let df: DataFrame = read_data_frame_from_csv(file_path);
    // print!("df head: \n{:#?}", df.head(Some(5)))


}
