Run by passing parameters of text file `cargo run -- items.txt`

Took some time to learn how to read from file, rather than hardcoding - you can pass any text file to confirm test.
Solution uses a combo of extracting numeric values and then finding 1st and 2nd number via .nth(0) + .nth_back(0)
.nth_back(0) in this case can also be the first number if there is only one number.

Not super challenging, but I was able to build some more comfort with Creates, Return Types - .parse().

--

Sooo. Apprently there was a part two which was slightly more difficult.
Not difficult in the sense of I had no idea how to do it
But rather, trying to iterate over the line and check for each word to replace with number
I couldn't quite work out how I should handle sequentially reading -> replace & then replace again if there was another number
In the end, rather than doing an if .contains .replace() - I found iterating over a tuple to be the best way to get to the end result
Convinced I had this solved, the number still wasn't showing as correct when submitting answer.
Double and triple checking the answer and code I couldn't understand why it wouldn't work.
I hit up google to see what the number SHOULD be and try to adjust code to somehow get it to work
Unfortunately I came across a reddit post that advised of the issue I was stuck at.
Essentially my code was returning words as oneight as (1eight) when it should have returned (18)
Solution I found for this was changing the variable replaced with to also include the surrounding letters of the number
I.E o1e, t2o, th3ee, fo4r, fi5e, s6x, se7en, ei8ht, n9e etc.
This, while probably not the best approached, allowed their niche cases to help cover any of those combining words.
Result found in the end.

Some things that came out of this that were helpful was getting some practice with the for (var1, var2) looping.
Rather than the initial if loop showning in lib.rs (commented out) which seems a bit more idiomatic.
A bit more comfortability with borrowing, but still some work for it to become second nature.

Keen to see others results.
-- End Day 01 --



--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
