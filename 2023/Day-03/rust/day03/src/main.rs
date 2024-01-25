use std::fs;


fn main() {
    println!("Hello, Day 03!");

    let items = array_file();
    // println!("{:?}", items);
    // array_lines(items.clone());
    let mut count = 0;
    count += check_surrounds(items);
    println!("{}", count);
    // for n in 1..items.len() {
    //     let prev = items[n-1].expect("Number out of range");
    //     let current = items[n];
    //     let next = items[n+1].expect("Number out of range");
    //     if n == 0 {
    //         count += check_surrounds(current,next);
    //     }
    //     else if n == items.len() {
    //         count += check_surrounds(prev,current);
    //     }
    //     else {
    //         count += check_surrounds(prev,current,next);
    //     }

        
    //     check_surrounds()
    //     println!("{}. line: {}", n, items[n]);
    // }
}


fn check_surrounds(lines: Vec<String>) -> i32 {
    // When a number is found we append it to temp_num
    // Any symbol found will trigger symbol_found to be true
    // If symbol_found = true we clear temp_num &  we don't append any numbers to temp_num
    // A found . will count += temp_num and reset temp_num to 0, also reset symbol_found to false
    // Repeat cycle

    let mut value = 0;
    let mut temp_num: String = "".to_string();
    let mut symbol_found = false; // Cheat was of getting the number, if it's false, it will 
    // if . && symbol_found { value += temp_num }
    // if symbol{ symbol_found = true; temp_num.clear(); }

    for i in 0..lines.len() {
        
        let mut previous: String = Default::default();
        if i == 0 {
            // println!("i is 0 so lines[1]");
            previous = lines[i].clone();  // !!! Need to catch case when i = 0 so we don't underflow backwards
            // println!("prev: {:?}", &previous);
        } else {
            previous = lines[i-1].clone();  // !!! Need to catch case when i = 0 so we don't underflow backwards
        }
        // println!("prev after if: {:?}", &previous);

        let current = lines[i].clone();

        let mut next: String = Default::default();
        if i == lines.len() -1 {
            next = lines[i].clone(); // !!! Need to catch case where length of lines. So we don't overflow.
        } else {
            next = lines[i+1].clone(); // !!! Need to catch case where length of lines. So we don't overflow.
        }
        // println!("Line: {}", lines[i]);

        let mut temp_temp_num: String = "".to_string();
        let mut char_index:i32 = 0;

        for c in lines[i].chars() {
            // println!("Temp Num on restart, with char_index: {}, {}")
 
            if c != '.' && !c.is_numeric() { symbol_found = true; } // This should catch cases where a number looks like 123&321 and reset temp_num
            if c.is_numeric() {
                temp_num.push_str(&c.to_string());
                // println!("ttn: {}", temp_num);

                if !symbol_found && any_symbols(previous.clone(), current.clone(), next.clone(), i as i32, char_index) {
                    
                    symbol_found = true;
                    // println!("Temp Num: {}, Symbol Found? {}", temp_num, symbol_found);
                }
                // println!("Char index: {}", char_index);
                // println!("Temp Num: {}", &temp_num);
                // println!("Line:{i} Char: {}", c);
                // println!("It's a number");
            }
            char_index += 1;
            // println!("Temp Num {:?}", &temp_num);
            if c == '.' || !c.is_numeric() {

                if temp_num.is_empty() { symbol_found = false;
                } else {                    
                    if symbol_found {
                        println!("Index: {} Adding: {:?} - {}", i, &temp_num, &symbol_found);
                        value += &temp_num.parse::<i32>().unwrap();
                    }
                    temp_num.clear();
                    symbol_found = false;
                }
                // println!("Found the dot");
            }
        }
        if !temp_num.is_empty() { // Once a line ends, we check to see if there was any valid number and then add the value to total. Otherwise we clear.
            if symbol_found{

                value += &temp_num.parse::<i32>().unwrap();
            }
            temp_num.clear();
            symbol_found = false;
        }  
        char_index = 0;
    }

    
    value
}

fn any_symbols(p: String, c: String, n: String, index: i32, char_index:i32) -> bool {
    let p_vec: &Vec<char> = &p.chars().collect();
    let c_vec: &Vec<char> = &c.chars().collect();
    let n_vec: &Vec<char> = &n.chars().collect();

    let mut c_ind = char_index as usize;
    if c_ind == 0              { c_ind = 1;}             // Handles UNDERFLOW
    if c_ind == c_vec.len() -1 { c_ind = &c_ind - 1; }   // Handles OVERFLOW
    // println!("What is vec len: {}", c_vec.len());
    // println!("c_ind is out: {}", &c_ind);
    // println!("\nIndex: {} - {} \np: {:?} \nc: {:?}, \nn: {:?} ", index, char_index, p,c,n);
    // println!("P should be {:?}", p_vec[char_index as usize -1]);
    // println!("Char usize: {}", char_index as usize);
    // Previous line Before, After and Next char
    if p_vec[c_ind - 1] != '.' && !p_vec[c_ind - 1].is_numeric() {
        return true
        // println!("There is a dot or symbol: {}", p_vec[c_ind - 1]);
    }
    if p_vec[c_ind] != '.' && !p_vec[c_ind].is_numeric() {
        return true
        // println!("There is a dot or symbol: {}", p_vec[c_ind]);
    }    
    if p_vec[c_ind + 1] != '.' && !p_vec[c_ind + 1].is_numeric() {
        return true
        // println!("There is a dot or symbol: {}", p_vec[c_ind + 1]);
    }
    


    // Current line Before & After char
    if c_vec[c_ind - 1] != '.' && !c_vec[c_ind - 1].is_numeric() {
        return true
        // println!("There is a dot or symbol: {}", c_vec[c_ind - 1]);
    }
    if c_vec[c_ind + 1] != '.' && !c_vec[c_ind + 1].is_numeric() {
        return true
        // println!("There is a dot or symbol: {}", c_vec[c_ind + 1]);
    }
    //

    // Next Line, Before After and Current char
    if n_vec[c_ind - 1] != '.' && !n_vec[c_ind - 1].is_numeric() {
        return true
        // println!("There is a dot or symbol: {}", n_vec[c_ind - 1]);
    }
    if n_vec[c_ind] != '.' && !n_vec[c_ind].is_numeric() {
        return true
        // println!("There is a dot or symbol: {}", n_vec[c_ind]);
    }
    if n_vec[c_ind + 1] != '.' && !n_vec[c_ind + 1].is_numeric() {
        return true
        // println!("There is a dot or symbol: {}", n_vec[c_ind + 1]);
    }
    return false
}



fn array_file() -> Vec<String> {
    let mut lines = Vec::<String>::new();


    for line in fs::read_to_string("input.txt").unwrap().lines() {
        lines.push(line.to_string());
    }
    for line in &lines{
        // println!("Lines: {:?}", line);
    }
    lines
}
// Plan of approach
// This is a peculiar one though I need to work out a couple of things
// 1. The full number, 123, 12, 8 etc. 2. The index of the first digit, second digit, third digit etc.
// To determine if a number is touching, I should probably pass three lines at a time to a function
// For Example:
// ....*..........
// ...321....123..
// ...............
// Since a number is detected on Line 2 at Index 3, I would need to check surrournding so:
// If Line 1 or Line 3 at index 3 == special character or line 2 at index 3 - 1 , we return false
// Otherwise we return true that there is no surrounding character
// We could essentially apply this logic to all numbers, though probably not the most efficient way to handle
// We are just checking above, below, left & right of the number
// If == special character then return false.
// 


// Or maybe we can just parse each character in order and check
// First split into their own arrays
// if contains_surround(item) { }
// 

// Actually maybe a Matrix would work here, though I'll need to learn this....
// Assuming a Matrix is just the full list of items, without splitting we can probably point to correspondance easiere. 
// Top left = 0,0 - bottom right = 100,140? Just like x,y
// 0 1 2 3 4 5
// 1 O X O O O
// 2 X 1 2 3 O
// 3 O X O O O
// 4 O O O O O
// 5 O O O O O
// so say 2,2 is a number we check 1,2 2,1, 2,3, 2,3  for a special characters (Marked by X)
// If not found we add that number to temp_variable temp = 1
// check next number at 3,2 -- 3,1, 3,3 
// 0 1 2 3 4 5
// 1 O O X O O
// 2 O 1 2 3 O
// 3 O O X O O
// 4 O O O O O
// 5 O O O O O
// Add to temp Variable = 12
// check next number at 4,2 -- 4,1, 5,2, 4,3 
// 0 1 2 3 4 5
// 1 O O O X O
// 2 O 1 2 3 X
// 3 O O O X O
// 4 O O O O O
// 5 O O O O O
// Add to temp variable = 123 then join += running count
// So essentially maximum of 4 checks for each number


// So maybe there isn't a matrix of sort available
// But as lines are same length we can just use array of arrays
// Same logic as above, but not using co-ords, we'd need to refer to the previous line and line after the current one being checked 
// THis is to beable to see if corrosponding index on the y-axis is matching to the currently checked number

// Something called grid from rows... might be worth looking into. Misunderstood exactly what a matrix was, probably just need coffee.


// Just realized, adjacent also allows for items diagonal to the number.
// So I'm thinking it may be worth going the route of:
// Determine the length of a number and performing a walk around 
// 
// > > > > V
// ^ 1 2 3 V
// ^ < < < <
//
// If a symbol is found, then we ignore this number and move to the next
// Otherwise we just add the number to the running `count`

// Walk around not even necessary, since it's a chunk - we would just search the current line -1 and +1 of the number index
// Do the same for the line above and below
// Add checks to make sure we don't go out of range on the index
// Should be solid

// Say we are on line 2 and we find a number at index of 3, when check the next space, if it's a number we include index of 4, if it's a Number we include index of 5
// Number takes up index[3], index[4], index[5] on line 2
// we need to check index[3-1], index[3], index[4], index[5], index[5+1] on line 1
// We need to check index[3-1], index[3], index[4], index[5], index[5+1] on line 2
// we need to check index[3-1], index[3], index[4], index[5], index[5+1] on line 3
// 
// We need to make sure the previous line exists, if not we ignore checking
// We need to make sure the proceeeding line exists, if not we ignore checking.
// These cases will only matter for the the very start and end of the input

// // So maybe we have fn check_surrounds(lines: Vec<Vec<&string>>, index: Vec<i8>) -> bool {
//     index = [3,4,5]
//     line1 = lines[0];
//     line2 = lines[1];
//     line3 = lines[2];

//     for line in line1[index[0], index[-1]+1] {
//         if symbol {
//             return true
//         }
//     // repeat for line 2/3
//     }
// }

// Alright so the above logic makes sense, there is probably a more efficient way to do it, but that seems like the route I would take.