use std::str::FromStr;

fn main() {
    // Read in the terminal output from the question trimming any surrounding whitespace
    let question_terminal_output = include_str!("../inputs/question").trim();

    // * Part One
    // <Whatever part one is doing>
    let part_one_result = part_one(question_terminal_output);
    println!("Part One:\n  The result is: {part_one_result}");

    // * Part Two
    // <Whatever part two is doing>
    let part_two_result = part_two(question_terminal_output);
    println!("Part Two:\n  The result is: {part_two_result}");
}

// region: Helpers

#[derive(Debug, PartialEq)]
enum FilesystemObject {
    // TODO: simplify this process by making Dir its own type so it can be passed around more specifically
    // - not needing to reconfirm whether something is a Dir vs a File
    Dir {
        name: String,
        children: Vec<FilesystemObject>,
    },
    File {
        name: String,
        size: u32,
    },
}

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
                FilesystemObject::Dir { name: dirname.to_string(), children: Vec::new() }
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

#[derive(Debug, PartialEq)]
enum Command {
    Cd(String),
    Ls,
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

#[derive(Debug, PartialEq)]
enum TerminalLine {
    FilesystemObject(FilesystemObject),
    Command(Command),
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

fn parse_terminal_output(terminal_output: &str) -> Vec<TerminalLine> {
    terminal_output
        .split('\n')
        .map(|line| TerminalLine::from_str(line).unwrap() )
        .collect::<Vec<TerminalLine>>()
}

fn get_dir<'a>(root: &'a mut FilesystemObject, path: &str) -> &'a mut FilesystemObject {
    let mut path_iter = path.split('/');
    // Handle preceeding slash of a path
    path_iter.next();

    path_iter
        .fold(root, |current_dir, path_level| {
            if path_level == "" {
                return current_dir;
            }
            // Find the directory with the path_level name inside current dir
            // If it doesn't exist, panic
            let FilesystemObject::Dir{
                name: _,
                children
            } = current_dir else {
                panic!("This should be a directory, but it wasn't");
            };

            children
                .iter_mut()
                .find(|filesystem_object| {
                    if let FilesystemObject::Dir { name, children: _ } = filesystem_object.clone() {
                        name == &path_level.to_string()
                    }
                    else {
                        false
                    }
                })
                .expect("Failed to find directory of the given path...")
        })
}

fn construct_filesystem(terminal_lines: Vec<TerminalLine>) -> FilesystemObject {
    let mut terminal_lines_iter = terminal_lines.into_iter();
    // Create the root element of the filesystem
    let mut root = if Some(TerminalLine::Command(Command::Cd("/".to_string()))) == terminal_lines_iter.next() {
        // The first command is changing to the root directory
        FilesystemObject::Dir {
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
                // println!("Current filesystem is:\n{:#?}", root);
                // println!("\nAttempting to find path: {working_dir_path}");
                // Get the current directory
                let FilesystemObject::Dir {
                    name: _,
                    children
                } = get_dir(&mut root, &working_dir_path)
                else {
                    panic!("Shouldn't get a file back from get_dir...")
                };

                // Add it to the current directory
                children.push(filesystem_object);
            },
            TerminalLine::Command(command) => match command {
                Command::Cd(relative_path) => {
                    if &relative_path == ".." {
                        // Remove one level from the end of the working_dir_path
                        working_dir_path = working_dir_path[..working_dir_path.rfind("/").unwrap()].to_string();
                    }
                    else {
                        // Add one level to the working_dir_path
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

fn determine_directory_sizes_from_terminal_output(terminal_output: &str) -> Vec<(String, u32)> {
    // Parse the output of the terminal into more helpful lines
    let terminal_lines = parse_terminal_output(terminal_output);

    // Construct the filesystem based on the lines
    let filesystem = construct_filesystem(terminal_lines);

    // Print out the filesystem
    // println!("The constructed filesystem is:\n{:#?}", filesystem);

    // Get the directory sizes list
    let mut directory_sizes = Vec::<(String, u32)>::new();
    get_dir_sizes(&filesystem, &mut directory_sizes);

    // Return the directory sizes
    directory_sizes
}

// endregion

// region: Part One

#[test]
fn part_one_example_test() {
    // Read in the example terminal output trimming any surrounding whitespace
    let example_terminal_output = include_str!("../inputs/example").trim();

    // <Part One goal> in the example input from the question
    let part_one_result = part_one(example_terminal_output);

    // Check if the example yields the same result as the question describes
    assert_eq!(part_one_result, 95437);
}

/// <Part One goal>
fn part_one(terminal_output: &str) -> u32 {
    // Parse the terminal output, construct the filesystem from it, and calculate the directory sizes... 
    let directory_sizes = determine_directory_sizes_from_terminal_output(terminal_output);

    // Filter the directories by those less than 10000 and sum them up
    directory_sizes
        .iter()
        .map(|(_, size)| size)
        .filter(|size| **size < 100000)
        .sum()
}

/// Given a filesystem dir and 
fn get_dir_sizes(dir: &FilesystemObject, directory_sizes: &mut Vec<(String, u32)>) -> u32 {
    // Check that it's a directory
    let FilesystemObject::Dir { name, children } = dir else {
        panic!("The object called as a directory is not actually a directory");
    };
    // Loop through its children
    let size = children
        .iter()
        .map(|filesystem_object| {
            // 
            match filesystem_object {
                FilesystemObject::Dir { name: _, children: _ } => get_dir_sizes(filesystem_object, directory_sizes),
                FilesystemObject::File { name: _, size } => *size,
            }
        })
        .sum();

    // Record the size of this directory in directory_sizes
    directory_sizes.push((name.clone(), size));

    // Return it so that its parents can use it too
    size
}

// endregion

// region: Part Two

#[test]
fn part_two_example_test() {
    // Read in the example terminal output trimming any surrounding whitespace
    let example_terminal_output = include_str!("../inputs/example").trim();

    // <Part Two goal> in the example input from the question
    let part_two_result = part_two(example_terminal_output);

    // Check if the example yields the same result as the question describes
    assert_eq!(part_two_result, 24933642);
}

/// <Part Two goal>
fn part_two(terminal_output: &str) -> u32 {
    // Parse the terminal output, construct the filesystem from it, and calculate the directory sizes... 
    let directory_sizes = determine_directory_sizes_from_terminal_output(terminal_output);

    // Print the dir sizes
    // println!("Dir sizes: {:?}", directory_sizes);

    // Get the total size of the filesystem
    let total_size = directory_sizes
        .iter()
        .find_map(|(name, size)| {
            if name == "/" {
                Some(size)
            }
            else {
                None
            }
        })
        .expect("Missing the root directory from list of directory sizes")
        .clone();

    // Determine the amount of space needed to be freed up
    //   space_needed_to_be_freed = total needed - (total space - current space)
    //   space_needed_to_be_freed = 30000000 - (70000000 - total_size);
    let space_needed_to_be_freed = total_size - 40000000;

    // Find the minimum directory size that is greater
    directory_sizes
        .iter()
        .map(|(_, size)| *size)
        .filter(|size| *size > space_needed_to_be_freed)
        .min()
        .unwrap()
}

// endregion
