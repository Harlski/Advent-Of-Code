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

        let mut temp_count = 0;
        let mut count = 0;

        for item in &game_numbers {
            for win_num in &winning_num {
                while item <= win_num {
                    if item.parse::<i8>().unwrap() == win_num.parse::<i8>().unwrap(){
                        // println!("Item: {:?} Win: {:?}", &item, &win_num);
                        count += 1;
                        temp_count += 1;
                    }
                    break
                }
            }
        }
        println! ("There was: {} matches", &count);
        let mut i = 1;
        while i <= count {
            // if (starting_index + 1) > {
            //     break;
            // }
            println!("What is len: {:?}",  array.len());
            // count_games(games_array[count as usize], )
            // println!("We would play: {} times.", count);
            println!("Current: {} \nNext: {} Total so far: {}", &starting_index, starting_index + i, &temp_count);
            println!("{}/{}", &i, &count);
            count_games(&array.clone(), &array[(starting_index + i) as usize], starting_index + i);
            if i == count {
                println!("Break");
                break;
            }
            // println!("Count: {}, SI: {}, SI+{}: {}", &count, &starting_index, &i, &starting_index + 1 + i);
            i += 1;
            // println!("Can we index into array: {:?}", array[count as usize]);
        }

        // for item in &game_numbers {
        //     for win_num in &winning_num {
        //         // println!("Compare I: {:?} W: {:?}", &item, &win_num);
        //         while item <= win_num {
        //             if item.parse::<i8>().unwrap() == win_num.parse::<i8>().unwrap(){
        //                 if count == 0 { count = 1; 
        //                 } else {
        //                     count = count * 2;
        //                 }
        //                 println!("Match! {:?}", item);
        //             } 
        //             break
        //         }
        //     }
        // }
        // line_count += linecount+1;
        // total_count += count;
        // println!("Count: {}", total_count);
        println!("//////////////////////////////////////? Returning: {}", &count);
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
