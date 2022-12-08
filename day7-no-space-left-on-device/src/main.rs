use std::str::FromStr;

fn main() {
    // Read in the terminal output from the question trimming any surrounding whitespace
    let question_terminal_output = include_str!("../inputs/question").trim();

    // * Part One
    // Find the total size of the directories larger than 100000 in the filesystem from the question
    let total_size_of_large_diretories = part_one(question_terminal_output);
    println!("Part One:\n  The result is: {total_size_of_large_diretories}");

    // * Part Two
    // Find the smallest possible directory that will give us enough space to update given the input from the question
    let size_of_directory_to_remove = part_two(question_terminal_output);
    println!("Part Two:\n  The result is: {size_of_directory_to_remove}");
}


// region: Terminal + Filesystem Types

#[derive(Debug, PartialEq)]
enum FilesystemObject {
    Dir(Directory),
    File {
        name: String,
        size: u32,
    },
}

#[derive(Debug, PartialEq)]
struct Directory {
    name: String,
    children: Vec<FilesystemObject>,
}

#[derive(Debug, PartialEq)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug, PartialEq)]
enum TerminalLine {
    FilesystemObject(FilesystemObject),
    Command(Command),
}

// endregion

// region: Input Parsing

impl FromStr for FilesystemObject {
    type Err = String;

    fn from_str(filesystem_object_str: &str) -> Result<Self, Self::Err> {
        // Examples:
        // dir a
        // 14848514 b.txt
        match filesystem_object_str
            .split_once(' ')
            .ok_or("The ls output is malformed - incorrect number of spaces".to_owned())? {
            ("dir", dirname) => Ok(
                FilesystemObject::Dir(Directory { name: dirname.to_string(), children: Vec::new() })
            ),
            (size, filename) => Ok(
                FilesystemObject::File {
                    name: filename.to_string(),
                    size: size.parse::<u32>().map_err(|_| format!("The size of the file has a bad format: '{size}'"))?,
                }
            )
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(command_str: &str) -> Result<Self, Self::Err> {
        // Examples:
        //  cd d
        //  ls
        let mut space_split = command_str.split(' ');
        match (space_split.next(), space_split.next()) {
            (Some("cd"), Some(dirname)) => Ok(Command::Cd(dirname.to_string())),
            (Some("ls"), None) => Ok(Command::Ls),
            _ => Err("Invalid command '{command_str}' in input".to_string()),
        }
    }
}

impl FromStr for TerminalLine {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // Examples:
        // $ cd /
        // $ ls
        // dir a
        // 14848514 b.txt
        if line.starts_with("$ ") {
            // It's a command. Parse the rest after '$ '
            Ok(TerminalLine::Command(Command::from_str(&line[2..])?))
        }
        else {
            // It's a filesystem object
            Ok(TerminalLine::FilesystemObject(FilesystemObject::from_str(line)?))
        }
    }
}

/// Given the `terminal_output` str, parse each line into their respective TerminalLine
fn parse_terminal_output(terminal_output: &str) -> Vec<TerminalLine> {
    terminal_output
        .split('\n')
        .map(|line| TerminalLine::from_str(line).unwrap() )
        .collect::<Vec<TerminalLine>>()
}

// endregion

// region: Helpers

fn get_dir<'a>(root: &'a mut Directory, path: &str) -> &'a mut Directory {
    let mut path_iter = path.split('/');
    // Handle preceeding slash of a path
    path_iter.next();

    path_iter
        .fold(root, |current_dir: &mut Directory, path_level| {
            // "Base case" if this is the end of the path
            if path_level == "" {
                return current_dir;
            }
            // Find the directory with the path_level name inside current dir
            current_dir.children
                .iter_mut()
                .find_map(|filesystem_object| {
                    match filesystem_object {
                        FilesystemObject::Dir(dir) if dir.name == path_level.to_string() => Some(dir),
                        _ => None,
                    }
                })
                .expect("Failed to find directory of the given path...")
        })
}

/// Constructs the root directory filesystem
fn construct_filesystem(terminal_lines: Vec<TerminalLine>) -> Directory {
    let mut terminal_lines_iter = terminal_lines.into_iter();

    // Create the root element of the filesystem
    let mut root = if Some(TerminalLine::Command(Command::Cd("/".to_string()))) == terminal_lines_iter.next() {
        // The first command is changing to the root directory
        Directory {
            name: "/".to_string(),
            children: Vec::new(),
        }
    }
    else {
        panic!("The first line was not moving to the root directory. I feel like it has to be.")
    };

    // Set the initial working directory to root
    let mut working_dir_path = "/".to_string();

    // Loop through the lines, constructing the file structure as you go
    for line in terminal_lines_iter {
        // println!("Handling line: {:?}", line);
        match line {
            TerminalLine::FilesystemObject(filesystem_object) => {
                // Seeing a filesystem object means it is an element of the current directory
                // Get the current directory
                let current_dir = get_dir(&mut root, &working_dir_path);

                // Add it to the current directory
                current_dir.children.push(filesystem_object);
            },
            TerminalLine::Command(command) => match command {
                Command::Cd(relative_path) => {
                    if &relative_path == ".." {
                        // Remove one level from the end of the working_dir_path
                        working_dir_path = working_dir_path[..working_dir_path.rfind("/").unwrap()].to_string();
                    }
                    else {
                        // Add one level to the working_dir_path handling the case of a trailing slash in the path
                        working_dir_path = format!(
                            "{working_dir_path}{}{relative_path}",
                            if working_dir_path.ends_with('/') { "" } else { "/" });
                    }
                    // println!("  New working_dir_path is: {working_dir_path}");
                },
                Command::Ls => continue, // doesn't really need to do anything as we are assuming any files we see are outputs of ls
            },
        }
    }

    root
}

/// Given a filesystem directory, calculate and record the sizes of the directories in the `directory_sizes` vec
fn calculate_directory_sizes(directory: &Directory, directory_sizes: &mut Vec<u32>) -> u32 {
    // Loop through its children, recursively finding the size of each, and summing up the total
    let total_size = directory.children
        .iter()
        .map(|filesystem_object| {
            match filesystem_object {
                FilesystemObject::Dir(subdirectory) => calculate_directory_sizes(subdirectory, directory_sizes),
                FilesystemObject::File { name: _, size } => *size,
            }
        })
        .sum();

    // Record the size of this directory in the directory_sizes vec
    directory_sizes.push(total_size);

    // Return it so that its parent can use it too
    total_size
}

/// Given the terminal output str, parse the lines, constructing the resulting filesystem, and calulcate the directory sizes within that filesystem
fn determine_directory_sizes_from_terminal_output(terminal_output: &str) -> (Vec<u32>, u32) {
    // Parse the output of the terminal into more helpful lines
    let terminal_lines = parse_terminal_output(terminal_output);

    // Construct the filesystem based on the lines
    let filesystem = construct_filesystem(terminal_lines);

    // Print out the filesystem
    // println!("The constructed filesystem is:\n{:#?}", filesystem);

    // Get the directory sizes list
    let mut directory_sizes = Vec::<u32>::new();
    let total_size = calculate_directory_sizes(&filesystem, &mut directory_sizes);

    // Return the directory sizes
    (directory_sizes, total_size)
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example terminal output trimming any surrounding whitespace
    let example_terminal_output = include_str!("../inputs/example").trim();

    // Find the total size of the directories larger than 100000 in the example filesystem from the question
    let total_size_of_large_diretories = part_one(example_terminal_output);

    // Check if the example yields the same result as the question describes
    assert_eq!(total_size_of_large_diretories, 95437);
}

/// Find the total size of the directories in the filesystem larger than 100000
fn part_one(terminal_output: &str) -> u32 {
    // Parse the terminal output, construct the filesystem from it, and calculate the directory sizes... 
    let (directory_sizes, _) = determine_directory_sizes_from_terminal_output(terminal_output);

    // Filter the directories by those less than 10000 and sum up their sizes
    directory_sizes
        .iter()
        .filter(|size| **size < 100000)
        .sum()
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example terminal output trimming any surrounding whitespace
    let example_terminal_output = include_str!("../inputs/example").trim();

    // Find the smallest possible directory that will give us enough space to update given the example input from the question
    let size_of_directory_to_remove = part_two(example_terminal_output);

    // Check if the example yields the same result as the question describes
    assert_eq!(size_of_directory_to_remove, 24933642);
}

/// Find the smallest possible directory size that will give us enough space to update if we were to remove it
fn part_two(terminal_output: &str) -> u32 {
    // Parse the terminal output, construct the filesystem from it, and calculate the directory sizes... 
    let (directory_sizes, total_size) = determine_directory_sizes_from_terminal_output(terminal_output);

    // Determine the amount of space needed to be freed up
    //   space_needed_to_be_freed = total needed - (total space - current space)
    //   space_needed_to_be_freed = 30000000 - (70000000 - total_size);
    let space_needed_to_be_freed = total_size - 40000000;

    // Find the minimum directory size that is greater than the amount of space needed to be freed up
    directory_sizes
        .iter()
        .cloned()
        .filter(|size| *size > space_needed_to_be_freed)
        .min()
        .unwrap()
}

// endregion
