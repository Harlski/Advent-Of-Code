use regex::Regex;
use std::fs;

pub fn contains_higher(games: Vec<Vec<String>>) -> bool {
    let (red_total, green_total, blue_total) = (12, 13, 14);

    for game in games {
        // println!("Game: {game_num}, - Draw: {:?}", game);
        let mut temp_num: i32 = 0;
    
        for item in game {
            if item.parse::<i32>().is_ok(){
                temp_num = item.parse::<i32>().expect("Shouldn't error.");
                // println!("It's a number mr white");
            }
            if item == "green" && temp_num > green_total { /* println!("Yo it's actually higher tho mr white"); */ return true }
            if item == "red" && temp_num > red_total { /* println!("Yo it's actually higher tho mr white"); */ return true }
            if item == "blue" && temp_num > blue_total { /* println!("Yo it's actually higher tho mr white"); */ return true }
            
            // println!("{item}");
        }
    }
    return false
}

pub fn file_to_vec() -> Vec<String> {

    // let re = Regex::new(r"\d+\s\w+(?:\,\s\d+\s\w+)*").unwrap(); // Regex to get numbers and color, may contain a `,` or may contain `;`.
    let mut items = Vec::<String>::new();
    
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        // println!("Adding item: {}", &line);
        items.push(line.to_string());
    }
    items
}   

pub fn split_lines(v: String) ->  (String, Vec<Vec<String>>) { // The idea here is to pass each line of file after regex and separate each drawing into its own array/vec.
    // let mut lines = Vec::<String>::new();
    let lines: Vec<_> = v.split(" ").collect();
    let game_num = return_game_num(lines[1]); // Just extracts the game number, which is always at index 1 of the line - however we run it through a function to remove the trailing `:`. "1:" => "1" etc.
    let games_array = return_array_of_games(lines); // Parses over everything after the `Game 1:`, it will create a temporary vector with number/color until it reaches a `;` when it sees this, it pushes the temp array and starts anew.
                                                    // This function also removes commas where detected, and when it reaches the end of the line it will push whatever is left into the array, as the last 'drawing' doesn't have a closing semi-colon 
                                                    

    fn return_game_num(number: &str) -> &str { // This function just removes the : from the end of the game number.
        // println!("In Number: {:?}", number);
        let num_cleaned = &number[0..number.len() -1];
        // println!("Out Number: {:?}", &number_rem);
        num_cleaned
    }

    fn return_array_of_games(line: Vec<&str>) -> Vec<Vec<String>> {  
        let mut out_vec = Vec::new();
        let mut temp_vec: Vec<String> = Vec::new();
        // println!("Line: {:?}", &line);
        for item in &line[2..] {
            // println!("Array from [2]: {:?}", item);
            if item.contains(",") { temp_vec.push(item[0..item.len() -1].to_string()); } // If there is comma, remove it and push value to the array
            else if item.contains(";") { temp_vec.push(item[0..item.len() -1].to_string()); out_vec.push(temp_vec.clone()); temp_vec.clear(); }  // If there is a semi-colon, remove it and push value to the array. Then push the array to the vector being returned & perform a clear on the temp vector.
            else { temp_vec.push(item.to_string()); } // If the item doesn't have either a comma or semi-colon, then we just add it to the array.
        }
        out_vec.push(temp_vec.clone()); // This is only reached once all items in array has been interated over, it pretty much just pushes the final temp_vec to out_vec, as the last drawing doesn't have a closing semi-colon to trigger the push.
        // println! ("Wassist: {:?}", out_vec);
        out_vec
    }
    // Still need to work out what to return
    // println!("Game: {:?} -- {:?}", game_num, games_array); // Proves that game number matches drawing and that drawings are split.
    (game_num.to_string(), games_array)
}
// Game 24: 12 red; 5 blue, 16 red; 2 blue, 1 green, 16 red; 1 green, 11 red; 2 blue, 8 red, 1 green


// \d+\s\w+(?:\,\s\d+\s\w+)*
// Plan of approach

// Determine is_impossible if count_red > 12, count_green > 13, count_blue > 14
// Input can be any amount of red/green/blue cubes
// Input may draw up to x amount of games
// Input can draw only one color or up to 3 1..3

// Need to parse and separate input in a way that game number is extracted
// First game is from : to ;
// Subsequent games are after ;
// With the final game having no trailing ;

// {Game [4]}: 
// {[ [1, green], [3,red], [6,blue] ]};
// {[ [3, green], [6, red] ]};
// {[ [3, green], [15, blue], [14, red] ]}

// Might need a struct {
//     "red": i32,
//     "green": i32,
//     "blue": i32
// }

// for loop over each drawing of the game
// if amount > total of color
// return true
// move to next game
// else loop ends, add game number to either Vec or just total as we go along 

// Might need to parse each drawing
// fn greater_than(color, amount) {
//     red = 12
//     green = 13
//     blue = 14

//     if amount > color.red{
//         return true
//     }
//     return false etc...
// }

// If totaling game number as we go along, no need for vec function
// Otherwise, we add the game number to a Vec<i32> then once all games check, 
// Call totale numbers on vec
// fn total_numbers(v: Vec<i32>) -> i32 {
//     let count = 0;
//     count += v.iter(); (?)
//     count
// } 

// So the logic sounds sound, but after experiencing day one - there will like be a second part
// Which will require some refactoring, so the thought is to probably got the vector route, just to make things less ambigous 