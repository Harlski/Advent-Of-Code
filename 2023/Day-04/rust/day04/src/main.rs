use std::fs;

fn main() {
    println!("Hello, Day04!");
   
    // let width = input.iter().position(|b| b == &b'\n').unwrap() as isize;
    // let mut temp: Vec<&str> = vec![];
    let mut winning_nums: Vec<&str> = vec![];
    let mut game_numbers: Vec<&str> = vec![];
    let mut total_count = 0;
    // println!("{:?}", (0..input.len() - 2).filter_map(|i| Some(input[*i] != b" ".swap())));
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let mut winning_num = line.split(":").last().unwrap().split("|").next().unwrap().split_ascii_whitespace().collect::<Vec<&str>>();
        let mut game_numbers = line.split(":").last().unwrap().split("|").last().unwrap().split_ascii_whitespace().collect::<Vec<&str>>();
        winning_num.sort_by_key(|x| x.parse::<i8>().unwrap());
        game_numbers.sort_by_key(|x| x.parse::<i8>().unwrap());


        let mut count = 0;
        for item in &game_numbers {
            for win_num in &winning_num {
                // println!("Compare I: {:?} W: {:?}", &item, &win_num);
                while item <= win_num {
                    if item.parse::<i8>().unwrap() == win_num.parse::<i8>().unwrap(){
                        if count == 0 { count = 1; 
                        } else {
                            count = count * 2;
                        }
                        println!("Match! {:?}", item);
                    } 
                    break
                }
            }
        }
        total_count += count;
        println!("Count: {}", total_count);
    }
}

// Bytes up to from 58(:) -> 124(|) -> 10(EOL)
// So the object of this seems pretty straight forward
// Though I expect with part 2 there is likely to be some spanner that causes it to not be so simple
// I want to try experimenting with .map() .filter() etc here as I noticed it to be a recurring tool used by other solves
// Day 03 I completed in like 500~ lines of code, compared to a more optimized version found of only 35~ lines....
// Using Tim Visée https://github.com/timvisee/advent-of-code-2023 as a reference/resource to compare answers.
// While Tim historically appears to be resolving these with speed in mind, realisticly it looks like something 
// That is worth discovering and understanding a bit more indepth.


// Challenge theory
// Potentially, I may just want to go the route of sorting both Winning Numbers and Game Numbers
// So I would assume an i over j iteration, i being wining numbers, j being game numbers
// If j is bigger than i then we move to the next item in i.
// If we find a match at any point then we update score * 2
// I don't know .map().filter() yet at this point but I assume it would be something like
// let winning_num = vec![].sorted() <--- I don't know best sorting method, so will need to look this up
// let game_numbers = vec![];
// let mut count:i32 = 0;

// game_numbers.iter_into().map(|x| { if}
// Some more to work out, this doesn't seem like a hard challenge
// The diffiuclty will likely come from me trying to understand new concepts.
// I could easily do this with if and for loops, but I really don't wnat to have to do that
// for every challenge, the hope is to learn something new each day of the puzzle
// But I feel like day 04 should be more difficult than day 3? Maybe theres something more to this that I haven't realized yet.
