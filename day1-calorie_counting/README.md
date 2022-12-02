# Advent of Code - Day 1 - Calorie Counting
See the quesiton for more details: https://adventofcode.com/2022/day/1

> Note: this program hardcodes the input I received in my Day 1 question.  If you wanted to generalize this program to use on any input, it would need to be changed to read in a relative path instead of directly including the input file in the binary.
> 
> If you wanted to dynamically read an input from a file (not including it in the compilation target), you could do so with something like this:
> ```rust
> let example_food_inventory = fs::read_to_string("inputs/example").expect("Failed to read in the file");
> ```
