fn main() {
    println!("Hello, Day 02!");
}

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