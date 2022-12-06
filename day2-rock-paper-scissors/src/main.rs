// ! Improvement: I think it might be better to change the characters to numbers: 0, 1, or 2 right at the beginning and work with them directly as those values
// It would simplify the logic in the rest and make what the strategy guide scoring functions are doing for both parts more clear/readable

fn main() {
    // Read in the strategy guide from the question trimming any surrounding whitespace
    let question_strategy_guide = include_str!("../inputs/question").trim();

    // * Part One
    // Determine the total score that would happen following the strategy guide described in part one
    let total_score = part_one(question_strategy_guide);
    println!("Part One:\n  The total score following the part one strategy guide is: {total_score}");
    
    // * Part Two
    // Determine the total score that would happen following the strategy guide described in part two
    let total_score = part_two(question_strategy_guide);
    println!("Part Two:\n  The total score following the part two strategy guide is: {total_score}");
}

// region: Helpers

/// Parse the given `strategy_guide` str into a list of each round's strategy guide represented by a pair of characters 
fn get_strategy_guide_per_round(strategy_guide: &str) -> Vec<(char, char)> {
    strategy_guide
        .split('\n')
        .map(|line| {
            // This is a kinda weird way to do this, but it works enough for the depth that I need to go for this challenge haha
            let (first_instruction, second_instruction) = line
                .split_once(' ')
                .expect("Missing a space to separate the columns -> bad input");
            let first_instruction = first_instruction.chars().next().expect("Missing a character in the first column -> bad input");
            let second_instruction = second_instruction.chars().next().expect("Missing a character in the second column -> bad input");

            // Return the pair of characters for the first and second instructions
            (first_instruction, second_instruction)
        })
        .collect::<Vec<(char, char)>>()
}

// Just a little python-inspired syntactic sugar :)
fn ord(char: char) -> i32 {
    Into::<u32>::into(char) as i32
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example strategy guide trimming any surrounding whitespace
    let example_strategy_guide = include_str!("../inputs/example").trim();

    // Determine the total score following the example strategy guide in part one's understanding of the guide
    let total_score = part_one(example_strategy_guide);

    // Check if the example yields the same result as the question describes
    assert_eq!(total_score, 15);
}

/// Get the sum of the scores per round that would happen following part one's understanding of the given `strategy_guide`
fn part_one(strategy_guide: &str) -> i32 {
    // Get the strategy guide per round for the opponent choice and the player choice
    let strategy_guide_per_round = get_strategy_guide_per_round(strategy_guide);

    // Determine the score for each round given the opponent choice and the player choice
    let score_per_round = strategy_guide_per_round
        .iter()
        .map(|(opponent_choice, my_choice)| {
            // Calculate the score of the given strategy considering the strategy interpretation of part one
            part_one_strategy_score(*my_choice, *opponent_choice)
        })
        .collect::<Vec<i32>>();

    // Return the sum of the scores per round
    score_per_round.iter().sum()
}

/// Calculate the score that would happen given the information we understand the strategy to contain according to part one
fn part_one_strategy_score(my_choice: char, opponent_choice: char) -> i32 {
    // let outcome = ((ord(my_choice) - 23) - ord(opponent_choice) + 4) % 3;
    let outcome_score = get_outcome(my_choice, opponent_choice) * 3;
    let shape_score = ord(my_choice) - ord('W');
    shape_score + outcome_score
}

/// Returns the matchup of two characters from the perspective of me, returning:
/// - `0` for a loss,
/// - `1` for a tie,
/// - `2` for a win
fn get_outcome(mine: char, opp: char) -> i32 {
    // Get the difference between the two plays
    let matchup_difference = (ord(mine) - 23) - ord(opp);

    // Shift the difference, wrapping around the result with modulo to handle each case
    (matchup_difference + 4) % 3
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example strategy guide trimming any surrounding whitespace
    let example_strategy_guide = include_str!("../inputs/example").trim();

    // Determine the total score following the example strategy guide in part two's understanding of the guide
    let total_score = part_two(example_strategy_guide);

    // Check if the example yields the same result as the question describes
    assert_eq!(total_score, 12);
}

/// Get the sum of the scores per round that would happen following part two's understanding of the given `strategy_guide`
fn part_two(strategy_guide: &str) -> i32 {
    // Get the strategy guide per round for the opponent choice and the required outcome
    let strategy_guide_per_round = get_strategy_guide_per_round(strategy_guide);

    // Determine the score for each round given the opponent choice and the required outcome
    let score_per_round = strategy_guide_per_round
        .iter()
        .map(|(opponent_choice, outcome)| {
            // Calculate the score of the given strategy considering the strategy interpretation of part two
            part_two_strategy_score(*opponent_choice, *outcome)
        })
        .collect::<Vec<i32>>();

    // Return the sum of the scores per round
    score_per_round.iter().sum()
}

/// Calculate the score that would happen given the information we understand the strategy to contain according to part two
fn part_two_strategy_score(opponent_choice: char, outcome: char) -> i32 {
    let outcome = ord(outcome) - ord('X');
    let outcome_score = outcome * 3;
    // Knowing the score of the opponent's shape, we can figure out our shape score will be depenging on what the outcome should be for this turn
    let shape_score = (ord(opponent_choice) - ord('A') + outcome + 2) % 3 + 1;
    outcome_score + shape_score
}

// endregion
