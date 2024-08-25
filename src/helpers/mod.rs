use colored::*;
use std::fmt::Display;

pub mod api;

//  a function to return the platform type , either windows , linux or mac if non return None
pub fn get_supported_platform() -> Option<String> {
    let os = std::env::consts::OS;
    match os {
        "windows" => Some("windows".to_owned()),
        "linux" => Some("linux".to_owned()),
        "macos" => Some("macos".to_owned()),
        _ => None,
    }
}




pub fn print_table<T>(header: Vec<&str>, data: Vec<Vec<T>>)
where
    T: Display,
{
    // Calculate the width of each column based on the longest item in each column
    let col_widths: Vec<usize> = header
        .iter()
        .enumerate()
        .map(|(i, &col_name)| {
            let max_data_width = data.iter().map(|row| row[i].to_string().len()).max().unwrap_or(0);
            std::cmp::max(max_data_width, col_name.len())
        })
        .collect();

    // Print the header with appropriate spacing and color
    let header_line: String = header
        .iter()
        .enumerate()
        .map(|(i, &col_name)| format!("{:^width$}", col_name.bold().cyan(), width = col_widths[i]))
        .collect::<Vec<String>>()
        .join(" | ");
    println!("{}", header_line);
    println!("{}", "-".repeat(header_line.len()));

    // Print each row of data with appropriate spacing
    for row in data.iter() {
        let row_line: String = row
            .iter()
            .enumerate()
            .map(|(i, value)| format!("{:^width$}", value, width = col_widths[i]))
            .collect::<Vec<String>>()
            .join(" | ");
        println!("{}", row_line);
    }
}
