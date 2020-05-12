use calamine::{Reader, Xlsx, open_workbook};
use multimap::MultiMap;

// File Libraries
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

fn main() {

    let path = "data/input.xlsx";

    // does file path exist?
    assert_ne!(Path::new(path).exists(), false);

    let _products = retrieve_data(path);

    
}

fn retrieve_data(path: &str) -> MultiMap<&str, String> {
    let mut map = MultiMap::new();

    // Open file
    let mut excel: Xlsx<_> = open_workbook(path).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open("data/out.csv")
        .unwrap();


    // Read whole worksheet, insert data
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {    
            map.insert("SKU", row[0].to_string());
            map.insert("Product Designation", row[1].to_string());
            map.insert("Unit Price (USD)", row[2].to_string());

            // write
            if let Err(e) = write!(file, "{},", row[0]) {
                eprintln!("Couldn't write to file: {}", e);
            }
            if let Err(e) = write!(file, "{},", row[1]) {
                eprintln!("Couldn't write to file: {}", e);
            }
            if let Err(e) = write!(file, "{},", row[2]) {
                eprintln!("Couldn't write to file: {}", e);
            }

            if let Err(e) = writeln!(file, "") {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }

    // make sure keys are properly added
    assert_eq!(map.contains_key("SKU"), true);
    assert_eq!(map.contains_key("Product Designation"), true);
    assert_eq!(map.contains_key("Unit Price (USD)"), true);

    return map;

}