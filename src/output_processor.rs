use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use crate::basic_types::*;

// Write the given data to the files
pub fn to_raw_binary_files(filename_prefix: &str, data: &FinishedRawData) {
    let mut img_file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(filename_prefix.to_string() + ".img.bin")
        .unwrap();

    match img_file.write_all(&data.image_data) {
        Ok(_) => {
            println!("Done writing file {}.img.bin", filename_prefix);
        }
        Err(_) => {
            error_out(format!("Error: Unable to write {}.img.bin", filename_prefix).as_str());
        }
    }

    if data.palette_data.len() > 0 {
        let mut pal_file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open(filename_prefix.to_string() + ".pal.bin")
            .unwrap();

        match pal_file.write_all(&data.palette_data) {
            Ok(_) => {
                println!("Done writing file {}.pal.bin", filename_prefix);
            }
            Err(_) => {
                error_out(format!("Error: Unable to write {}.pal.bin", filename_prefix).as_str());
            }
        }
    }
}
