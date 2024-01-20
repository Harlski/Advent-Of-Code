use std::fs;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>>  {
    let mut items = Vec::new();
    let mut count = 0;
    for line in fs::read_to_string(config.file_path).unwrap().lines() {
        println!("Adding item: {}", &line);
        items.push(line.to_string());
    }

    for item in items {
        count += count_nums(item);
    }
    println!("Total count: {}", count);
    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>  {
        if args.len() < 2{
            return Err("Not enough args");
        }
        let file_path = args[1].clone();
    
        Ok(Config { file_path })
    }
}

pub struct Config {
    pub file_path: String,
}

fn count_nums(v: String) -> i32 {
    let mut count = 0;

    count += total_first_and_last(v);
    fn total_first_and_last(v: String) -> i32 {
        let result: String = v.chars().filter(|c| c.is_numeric()).collect(); // All Numbers In Line
        println!("Result: {:?}", result);

        let num1 = result.chars().nth(0).unwrap();
        let num2 = result.chars().nth_back(0).unwrap();
        let concat = num1.to_string() + &num2.to_string();
        // println!("Num1: {:?}, Num2: {:?} with Concat: {:?}", &num1, &num2, &concat);
        let combined: i32 = concat.parse().expect("Some issue");
        return combined
    }

    return count
}