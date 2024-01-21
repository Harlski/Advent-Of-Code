use day02;

fn main() {
    println!("Hello, Day 02!");
    let lines = day02::file_to_vec();
    let mut total_count:i32 = 0;
    
    for line in lines {
        let (game_num, games) = day02::split_lines(line); 
        if !day02::contains_higher(games) {
            total_count += game_num.parse::<i32>().expect("No errors");
        }
    }
    println!("Answer is: {}", total_count);
}
// println!("Lines: {:?}", &lines);

