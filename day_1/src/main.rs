use std::fs::read_to_string;
use std::env;

//RIGHT ANSWER = 54644
fn get_line_calibration(line: String) -> u32 {
    let mut calibration: u32 = 0;
    
    //First digit must be multiplied by 10 so that it creates a double digit number
    //as asked in the exercise
    calibration += 10 * line
    .chars()
    .find(|c| c.is_numeric())
    .and_then(|c| c.to_digit(10))
    .unwrap_or_default();

    calibration += line
    .chars()
    .rev()
    .find(|c| c.is_numeric())
    .and_then(|c| c.to_digit(10))
    .unwrap_or_default();

    // // Another less elegant way to solve this exercise
    // for c in line.chars() {
    //     if c.is_numeric() {
    //         if let Some(digit) = c.to_digit(10) {
    //             calibration += digit * 10;
    //             break;
    //         }
    //     }
    // }

    // for c in line.chars().rev() {
    //     if c.is_numeric() {
        //         if let Some(digit) = c.to_digit(10) {
    //             calibration += digit;
    //             break;
    //         }
    //     }
    // }
    
    calibration
}

fn get_calibration(file_path: &str) -> u32 {
    let mut calibration: u32 = 0;

    //Extract calibration line by line
    for line in read_to_string(file_path).unwrap().lines() {
        calibration += get_line_calibration(line.to_string());
    }

    calibration
}

//call like this `cargo build && cargo run -- ./file/path.txt`
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let calibration = get_calibration(file_path);

    println!("Calibration for this file is : {calibration}. Let's make it snoooow !");
}