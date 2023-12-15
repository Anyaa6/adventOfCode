use std::collections::HashMap;
use std::fs::read_to_string;
use std::env;

#[allow(unused_variables)]

fn get_first_digit(line: String) -> u32 {

    //Need to also go from both ends in case line is super long 
    //Find first digit
    //Use find() on string, return index AND corresponding number
    //compare which index comes first -> add this one * 10
        
    let calibration += 10 * line
    .chars()
    .find(|c| c.is_numeric())
    .and_then(|c| c.to_digit(10))
    .unwrap_or_default();

    calibration
}

fn get_last_digit(line: String) -> u32 {
    //Find last digit
        //Use rfind() on string
        //compare which index comes first -> add this one
    let calibration += line
    .chars()
    .rev()
    .find(|c| c.is_numeric())
    .and_then(|c| c.to_digit(10))
    .unwrap_or_default();

    calibration
}

fn get_line_calibration(line: String) -> u32 {
    let mut calibration: u32 = 0;

    calibration += get_first_digit(line);
    calibration += get_last_digit(line);

    calibration
}

const NUMBER_MAP: HashMap<&str, u8> = HashMap::from([
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ]);

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut calibration: u32 = 0;
        
    //Extract calibration line by line
    for line in read_to_string(file_path).unwrap().lines() {
        println!("{line}");
        calibration += get_line_calibration(line.to_string());
    }
    
    println!("Calibration for this file is : {calibration}. Let's make it snoooow !");
}
