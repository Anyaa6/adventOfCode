use std::fs::read_to_string;
use std::env;

//RIGHT ANSWER = 54644
fn get_line_calibration(line: String) -> u32 {
    let mut calibration: u32 = 0;
    
    for c in line.chars() {
        if c.is_numeric() {
            if let Some(digit) = c.to_digit(10) {
                calibration += digit * 10;
                break;
            }
        }
    }
    
    for c in line.chars().rev() {
        if c.is_numeric() {
            if let Some(digit) = c.to_digit(10) {
                calibration += digit;
                break;
            }
        }
    }
    
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