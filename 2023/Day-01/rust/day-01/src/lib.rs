use std::fs;
use std::error::Error;

pub struct Config {
    pub file_path: String,
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>>  {
    // Add each line to a Vec/Array.
    let mut items = Vec::new();
    for line in fs::read_to_string(config.file_path).unwrap().lines() {
        println!("Adding item: {}", &line);
        items.push(line.to_string());
    }
    // Count numbers
    let mut count = 0;
    for item in items {
        println!("Count: {} In: {}", &count, &item);
        count += count_nums(item);
    }
    println!("Total count: {}", count);
    Ok(())
}
    


fn count_nums(v: String) -> i32 {
    
    let mut count = 0;
    let v = with_replaced_nums(&v);
    println!("After: {}", &v);

    count += combine_first_and_last(v);
    fn combine_first_and_last(v: String) -> i32 {
        let result: String = v.chars().filter(|c| c.is_numeric()).collect(); // All Numbers In Line
        // println!("Result: {:?}", result);
        let num1 = result.chars().nth(0).unwrap();
        let num2 = result.chars().nth_back(0).unwrap();
        
        // println!("First: {}, Last: {}", &num1, &num2);
        let concat = num1.to_string() + &num2.to_string();
        // println!("Num1: {:?}, Num2: {:?} with Concat: {:?}", &num1, &num2, &concat);
        let combined: i32 = concat.parse().expect("Some issue");
        println!("Combined: {}", &combined);
        return combined
    }
    
    return count
}

fn with_replaced_nums(v: &str) -> String {
    let mut result = check_for_num(v);
    println!("Before: {}", &v);

    fn check_for_num(v: &str) -> String {
        let patterns = [("one","o1e"),("two","t2o"),("three","th3ee"),("four","fo4r"),("five","fi5e"),("six","s6x"),("seven","se7en"),("eight","ei8ht"),("nine","n9e")];
        let mut temp_result = v;
        // println!("Temp Result: {}", &temp_result);

        let mut temp: String = (&temp_result).to_string();
        for (text, num) in patterns {
            if temp_result.contains(&text){
                temp = temp_result.replace(text,num).to_string();
                temp_result = &temp;
                // println!("Doing {:?} : {:?}", text, num);
                // println!("Temp: {}", temp_result.replace(text,num));
                // println!("Result: {} Contains {}", result, text);
                // println!("result: {}", &temp_result);
                // temp_result = temp_temp_result;
            }
            // println!("temp_r + temp: {} {} ", &temp_result, &temp);
            // temp_result = &temp;
        }
        temp_result.to_string()
    }
    result.to_string()
}
    
    // println!("Resultresult: {}", &result);
    // if result.contains("one") { result.replace("one", "1"); }
    // if result.contains("two") { result.replace("two", "2"); }
    // if result.contains("three") { result.replace("three", "3"); } 
    // if result.contains("four") { result.replace("four", "4"); }  
    // if result.contains("five") { result.replace("five", "5"); } 
    // if result.contains("six") { result.replace("six", "6");  } 
    // if result.contains("seven") { result.replace("seven", "7"); }
    // if result.contains("eight") { result.replace("eight", "8"); }
    // if result.contains("nine") { result.replace("nine", "9"); } 