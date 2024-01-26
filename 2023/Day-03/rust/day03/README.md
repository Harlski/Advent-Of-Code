// END DAY 03 PART 02
What a wild one, by no means do I think the answer here is idiomatic.
There is so much ramblings scatted amongst this code, things that lead to no where - println statements everywhere to check every single step

While I feel like maybe I am getting slightly better with Rust, there is still some ways to go - but at this stage I'm not getting de-motivated to stop at all. 

Everyday I'm coming back to do some practice, I don't even know how long I've spent in total - I know I am commiting to at least 1 hour per day - but I feel like at this stage I'm probabably on par with 4 hours per day.

I am really enjoying reading challenge outlines and sleeping on it, regularly thinking about the problem and writing in notebook on possible solution paths

For the most part I'm enjoying the problem solving, but I would really love to expand and use some other inbuilt methods.

Some of the previous challenges I've looked at AFTER solving, I've found that .map is a recurring theme - I don't understand this at all, but I might look into understanding its uses and we may see it used in future challenges.

I don't know what Day 04 lies just yet, but they're obviously getting a bit more difficult. 

Onwards!



// Part 2 Theory
// So I'm probably going to have to refactor everything now, as the current setup won't work in tracking two numbers
// The plan is likely to find the index of all symbols, perform a search around it to see if two single numbers are detected.
// Somehow iterate over the adjacent line indexes to get the full numbers and then perform a multiply + add to total
// Likely path is: Find Symbol > Find a number > Look left and right, if number we add it to a variable. When a . or symbol is detected  
// We consider the number found

// Going to sleep on it, but breaking it down it shouldn't be too difficult.

// Conditions where a match (two Numbers) is true

... | 1.1 1.. 1.. 1.. 1.. 1.. .1. .1. .1. .1. .1. ..1 ..1 ..1 ..1 ..1 ... ... ... ... ... ... ... ...
.&. | .&. 1&. .&1 .&. .&. .&. 1&. .&1 .&. .&. .&. 1&. .&1 .&. .&. .&. 1&1 1&. 1&. 1&. .&1 .&1 .&1 .&.           
... | ... ... ... 1.. .1. ..1 ... ... 1.. .1. ..1 ... ... 1.. .1. ..1 ... 1.. .1. ..1 1.. .1. ..1 1.1             

// I don't know if I need to hardcode each of these conditions 
// Two numbers on a separate line is easy to confirm, as it's guarenteed that it's two different numbers.
// So maybe I'll split them...

I think the first two conditions I need to check to make sure there is a gap between the two numbers, 
This is the crucial identifier to determine if it's two individual numbers or one three digit number
The third one I just need to confirm if current[i-1] && current[i+1] is a number, this identifies two individual.

1.1 ... | ...
.&. .&. | 1&1
... 1.1 | ...

Any other arrangement should ideally determine that the gear is touching two numbers. 
Maybe we do bool num_on_one, num_on_two, num_on_three
... | 1.. 1.. 1.. 1.. 1.. .1. .1. .1. .1. .1. ..1 ..1 ..1 ..1 ..1  ... ... ... ... ... ... 
.&. | 1&. .&1 .&. .&. .&. 1&. .&1 .&. .&. .&. 1&. .&1 .&. .&. .&.  1&. 1&. 1&. .&1 .&1 .&1            
... | ... ... 1.. .1. ..1 ... ... 1.. .1. ..1 ... ... 1.. .1. ..1  1.. .1. ..1 1.. .1. ..1    

First we check the three conditions to confirm separate numbers, otherwise
If bool is true on two lines, then we return the index and save it for processing later

When we come back to process later we pass through the index + the two surrounding lines 

.....797
...*....
502.118.

We identify number at n[i-1] and number at n[i+1]
first_num = "2"
(maybe while? )if n[i-2].is_numeric {
    first_num ="02"
} if n[i-3].is_numeric {
    first_num = "502"
} if n[i-4].is_numerc{
    !!not number!!
}

Same for right side but plus instead


sec_num = "1"
(maybe while? )if n[i+2].is_numeric {
    sec_num ="11"
} if n[i+3].is_numeric {
    sec_num = "118"
} if n[i+4].is_numerc{
    !!not number!!
}

return first_num * second_num


                /* Here we need to confirm the remaining cases 
                ... | 1.. 1.. 1.. 1.. 1.. .1. .1. .1. .1. .1. ..1 ..1 ..1 ..1 ..1  ... ... ... ... ... ... 
                .&. | 1&. .&1 .&. .&. .&. 1&. .&1 .&. .&. .&. 1&. .&1 .&. .&. .&.  1&. 1&. 1&. .&1 .&1 .&1            
                ... | ... ... 1.. .1. ..1 ... ... 1.. .1. ..1 ... ... 1.. .1. ..1  1.. .1. ..1 1.. .1. ..1    

                The bool logic just below declaration of number1 & number2 should be used here. Matching cases are:
                num_line_1 && num_line_3
                -- This case should have start at index and do a while check left and right. While numeric
                -- While index-i is true, we add to number.
             
                .233...
                ...&...
                ...344.

                iter..previous
                number1 = 3
                number1 = 23
                number1 is .
                while index+i is numeric
                number1 is 233
                number1 is .

                iter..next
                number2 = 4
                number2 = 44
                index+i is .
                while index-i is numeric
                number2 == 344
                index-1 is .

                number1 * number2
                return combined 

                num_line_1 && num_line_2
                num_line_2 && num_line 3

                */




--- Day 3: Gear Ratios ---

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

