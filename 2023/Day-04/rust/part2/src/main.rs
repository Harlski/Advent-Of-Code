use std::fs;

fn main() {
    println!("Hello, Day04 Part 2!");
   
    // let width = input.iter().position(|b| b == &b'\n').unwrap() as isize;
    // let mut temp: Vec<&str> = vec![];
    let mut games_array: Vec<String> = vec![];
    let mut winning_nums: Vec<&str> = vec![];
    let mut game_numbers: Vec<&str> = vec![];
    let mut total_count = 0;
    // println!("{:?}", (0..input.len() - 2).filter_map(|i| Some(input[*i] != b" ".swap())));
    let mut line_count: i8 = 0;
    let mut index: i32 = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        games_array.push(line.to_string());

    }

    for item in games_array.clone() {
        println!("Count is: {}, Array Len: {}", index, &games_array.len());
        total_count += count_games(&games_array, &item.clone(), index);
        index += 1;
        println!("Answer should be: {}", &total_count);

    }
    // println!("Games Array: {:?}", games_array);

    fn count_games(array: &Vec<String>, line: &str, starting_index: i32) -> i32 {
        println!("[{}]", starting_index);
        let mut winning_num = line.split(":").last().unwrap().split("|").next().unwrap().split_ascii_whitespace().collect::<Vec<&str>>();
        let mut game_numbers = line.split(":").last().unwrap().split("|").last().unwrap().split_ascii_whitespace().collect::<Vec<&str>>();
        winning_num.sort_by_key(|x| x.parse::<i8>().unwrap());
        game_numbers.sort_by_key(|x| x.parse::<i8>().unwrap());
        let mut count = 0;
        
        game_numbers.iter().for_each(|x| { for item in winning_num.clone() { if *x == item { println!("Match: {}", item); count += 1; }} });

        let mut i: i32 = 1;
        while i <= count {
            println!("{i}");
            count_games(array, &array[(starting_index + i) as usize],  starting_index + i);
            i += 1;
        }
        // println!("{:?} | {:?}", winning_num, game_numbers);

        count
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
