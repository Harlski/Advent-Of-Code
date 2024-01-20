use std::env;
use std::process; 
use day01::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem passing args: {err}");
        process::exit(1);
    });
    
    println!("In file: {}", config.file_path);
    if let Err(e) = day01::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}
//     calculate_values(input_file);
// }

// fn calulate_values(textfile) -> i32 {
//     let mut total:i32 = 0;
//     fn parse_file(txtfile) -> Vec[i32] {
//         // Need to work out if we need this function, or wheter we can just pass each ine of fille to r_f_a_l();
//         return v
//     }

//     fn return_first_and_last(string: &str) -> i32 {
//         //Find the first and last number, if only one number is found, num1 = num2
//         //Return the sum of two numbers
//         return num1 + num2; 
//     }

//     items = parse_file(textfile);

//     for item in items{
//         total += return_first_and_last(item);
//     }
//     return total;
// }







































// --- Day 1: Trebuchet?! ---

// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

// For example:

// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet

// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

// Consider your entire calibration document. What is the sum of all of the calibration values?
