use day02;

fn main() {
    println!("Hello, Day 02!");
    let lines = day02::file_to_vec();
    let mut total_count:i32 = 0;
    
    // PART 1
    // for line in lines {
    //     let (game_num, games) = day02::split_lines(line); 
    //     if !day02::contains_higher(games) {
    //         total_count += game_num.parse::<i32>().expect("No errors");
    //     }
    // }

    // PART 2
    for line in lines {
        let (_game_num, games) = day02::split_lines(line); 
        total_count += day02::fewest_number(games);
    }
    println!("Answer is: {}", total_count);
}
// println!("Lines: {:?}", &lines);

