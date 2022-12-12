use std::iter;

fn main() {
    // Read in the tree heights from the question trimming any surrounding whitespace
    let question_tree_heights = include_str!("../inputs/question").trim();

    // * Part One
    // Count the number of visible trees from in the grid input from the question
    let visible_tree_count = part_one(question_tree_heights);
    println!("Part One:\n  The number of visible trees is: {visible_tree_count}");

    // * Part Two
    // Find the maximum scenic score among the trees in the tree grid input from the question
    let max_scenic_score = part_two(question_tree_heights);
    println!("Part Two:\n  The maximum scenic score among all the trees is: {max_scenic_score}");
}

// region: Helpers

/// Parse the tree grid input into the respective tree heights
fn parse_tree_heights(tree_heights_str: &str) -> Vec<Vec<u8>> {
    tree_heights_str
        .split('\n')
        .map(|line| {
            line
                .chars()
                .map(|char_num| char_num.to_digit(10).expect("Non-number height in input") as u8)
                .collect()
        })
        // Note: collect type is inferred from the return type of the function
        .collect()
}

fn tree_directional_map<F>(x: usize, y: usize, tree_heights: &Vec<Vec<u8>>, map_fn: F) -> Vec<u32>
where
    F: Fn(u8, Vec<u8>) -> u32, {
    // Get the height of the current tree
    let current_tree_height = tree_heights[y][x];

    // I feel like a logical extension for part two would be diagonal visibility as well, so I'll make it this way in case :)
    // Check up, down, left, right
    let directions = [(0, 1), (0, -1), (-1, 0), (1, 0)];

    // For each direction from the tree, map the iterator in that direction to a number
    directions
        .iter()
        .map(|direction| {
            // Add the state to be used by a direction iterator
            let mut position = (x as i32, y as i32);
            // Construct an iterator that yields the heights of the trees in a given direction
            let direction_iter = iter::from_fn(|| {
                // Update the position of what the next tree in the iter would be
                position = (position.0 + direction.0, position.1 + direction.1);

                // Get the height of the next tree, passing along None to the iter if the index is out of bounds on either side
                Some(*tree_heights
                    .get(usize::try_from(position.1).ok()?)?
                    .get(usize::try_from(position.0).ok()?)?
                )
            }).collect::<Vec<u8>>();

            // Return the value the map_function for the given direction
            map_fn(current_tree_height, direction_iter)
        })
        .collect::<Vec<_>>()
}

/// Given the grid of `tree_heights`, evaluate each tree's ordinal surroundings with the evaluating it by the given `evaluate_fn`
fn evaluate_tree_surroundings<F>(tree_heights: &Vec<Vec<u8>>, (height, width): (usize, usize), evaluate_fn: F) -> Vec<Vec<u32>>
    where F: Fn(usize, usize, &Vec<Vec<u8>>) -> u32 {
    (0..height).map(|y| {
        (0..width).map(|x| {
            // Determine if the tree at (x, y) satisfies the evaluate_fn
            evaluate_fn(x, y, &tree_heights) as u32
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()

    // Iterative approach
    /*
    let mut visibility = vec![vec![0; width]; height];
    for y in 0..height {
        for x in 0..width {
            // Determine if the tree at (x, y) is visible
            visibility[y][x] = evaluate_fn(x, y, &tree_heights) as u32;
        }
    } */
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example tree heights trimming any surrounding whitespace
    let example_tree_heights = include_str!("../inputs/example").trim();

    // Count the number of visible trees from in the example grid input from the question
    let visible_tree_count = part_one(example_tree_heights);

    // Check if the example yields the same result as the question describes
    assert_eq!(visible_tree_count, 21);
}

/// Count the number of trees in the grid that are visible from outside the grid
fn part_one(tree_heights_input: &str) -> u32 {
    // Parse the heights of the trees as u8 from the input tree grid
    let tree_heights = parse_tree_heights(tree_heights_input);

    // Determine the height and width of the tree grid
    let height = tree_heights.len();
    let width = tree_heights.first()
        .expect("Missing first line of the tree grid - invalid input")
        .len();

    // Evaluate each tree on whether it is visible from the outside
    let visibility = evaluate_tree_surroundings(&tree_heights, (height, width), tree_is_visible);

    // Sum the number of visible trees
    visibility.iter().flatten().sum()
}

/// Return the visibility of the tree as (`x`, `y`) from the edge from any cardinal direction as an int
fn tree_is_visible(x: usize, y: usize, tree_heights: &Vec<Vec<u8>>) -> u32 {
    // Traverse the surrounding trees
    let direction_visibility = tree_directional_map(x, y, tree_heights, tree_visibile_in_direction);

    // The tree is visible if it is visible from any of the directions
    // Note: since 'any' is short-circuiting, it would be better performance-wise to not collect it into a vector above and just call any
    //  on the initial iter. I like the readability here though, and it might make more sense depening on the requirements from part two
    direction_visibility.iter().any(|visible| *visible == 1) as u32
}

/// Return whether the tree is visible from this direction by checking if there arent
///  any trees obstructing it (that is, taller than or equal to it)
/// 
/// Doing it this way also works for edges as well as `any_obstructing` will be false if there is nothing in that direction
fn tree_visibile_in_direction(current_tree_height: u8, direction_heights: Vec<u8>) -> u32 {
    let any_obstructing = direction_heights
        .iter()
        .any(|tree_height| *tree_height >= current_tree_height);
    !any_obstructing as u32
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example tree heights trimming any surrounding whitespace
    let example_tree_heights = include_str!("../inputs/example").trim();

    // Find the maximum scenic score among the trees in the example tree grid input from the question
    let max_scenic_score = part_two(example_tree_heights);

    // Check if the example yields the same result as the question describes
    assert_eq!(max_scenic_score, 8);
}

/// Find the maximum scenic score among all of the trees in the grid
fn part_two(tree_heights_input: &str) -> u32 {
    // Parse the heights of the trees as u8 from the input tree grid
    let tree_heights = parse_tree_heights(tree_heights_input);

    // Determine the height and width of the tree grid
    let height = tree_heights.len();
    let width = tree_heights.first()
        .expect("Missing first line of the tree grid - invalid input")
        .len();

    // Evaluate each tree on what its scenic score is
    let scenic_scores = evaluate_tree_surroundings(&tree_heights, (height, width), tree_scenic_score);

    // Find the maximum scenic score
    *scenic_scores.iter().flatten().max().expect("Failed to find a max scenic score...")
}

/// Calculate the scenic score of the tree at (`x`, `y`) considering each cardinal direction
fn tree_scenic_score(x: usize, y: usize, tree_heights: &Vec<Vec<u8>>) -> u32 {
    // Traverse the surrounding trees
    let direction_scores = tree_directional_map(x, y, tree_heights, tree_scenic_score_in_direction);

    // Get the total scenic score as the product of each direction's score
    direction_scores.iter().product()
}

/// Calculate the driectional scenic score for a tree of height `current_tree_height` using the heights of the trees in the given direction 
fn tree_scenic_score_in_direction(current_tree_height: u8, direction_heights: Vec<u8>) -> u32 {
    let mut score = 0;
    for tree_height in direction_heights {
        // If our view is not yet obstructed, increment the scenic score for this tree
        score += 1;
        // If this tree obstructs the view, we won't be able to see any more
        if tree_height >= current_tree_height { 
            break;
        }
    }
    // Return the resulting scenic score
    score
}

// endregion
