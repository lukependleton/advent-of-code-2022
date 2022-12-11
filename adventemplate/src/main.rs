use std::{
    process::Command,
    path::Path,
    fs::{File, self},
    io,
};

use clap::Parser;
use colored::{Colorize, ColoredString};
use tera::{Tera, Context};

// Define program input with clap
#[derive(Parser)]
struct Adventemplinput {
    #[arg(help = "The number of the day of Advent of Code it is.")]
    day_num: u8,

    #[arg(help = "The terminology used to describe the input for the question used in templating the main.rs file.")]
    input_structure_name: Option<String>,
}

fn main() {
    // Joy to the world!
    println!(
        "\nHello and {} {} {} {}{}! Let's template out some Advent of Code boilerplate solutions!",
        "joy".bright_yellow().bold(),
        "to".red(),
        "the".green(),
        "wor".red(), "ld".green(),
    );

    // * Parse input arguments with clap
    // Steps we can choose to do or not:
    //  - cargo new (needs an input file)
    //  - template main file
    //  - add inputs dir
    //  - add gitignore
    //  - add README
    // Possible ways to configure what steps run - all, whitelist, blacklist

    // Parse with clap
    let adventemplinput = Adventemplinput::parse();

    // TODO: prompt for input_structure_name if it is not included
    // ? Perhaps we can provide them with the text block of the article description if they want to be able to read it there
    let input_structure_name = adventemplinput.input_structure_name.expect("Missing input_structure_name. For now, that's kinda required");
    // let input_structure_name = String::from("datastream_buffer");j

    // TODO: given the day, request and parse the page for the day and get the title & project name
    let year = "2022";
    let day_num = adventemplinput.day_num;
    println!("\nLooking up info for day {day_num}...");
    let day_info = get_day_title_and_project_name(year, day_num, input_structure_name);
    println!(
        "  Title found for day {day_num}:\n    {}",
        day_info.day_title.bold()
        // day_info.day_title.bold().on_truecolor(80, 40, 90).truecolor(150, 140, 10)
    );

    // Definte the possible steps
    let template_steps = [
        SetupStep {
            step_message: String::from("Creating new rust project with cargo..."),
            step_executor: create_rust_project,
            critical: true,
        },
        SetupStep {
            step_message: String::from("Adding gitignore to project..."),
            step_executor: add_gitignore,
            critical: false,
        },
        SetupStep {
            step_message: String::from("Creating README for project..."),
            step_executor: create_readme,
            critical: false,
        },
        SetupStep {
            step_message: String::from("Creating inputs folder..."),
            step_executor: create_inputs,
            critical: false,
        },
        SetupStep {
            step_message: String::from("Templating main.rs file..."),
            step_executor: template_main_rs,
            critical: false,
        },
    ];

    // We could filter the template_steps based on the input...

    // * Execute the steps specified by the input arguments
    for step in template_steps {
        println!("\n--------------------");
        println!("\n{}", step.step_message.blue());
        // ? Maybe make all text outputted from the step execution indented
        match (step.step_executor)(&day_info) {
            Ok(()) => println!("{}", "Success!".green()),
            Err(e) => {
                println!("{}", format!("Failed with error: '{e}'").red());
                if step.critical {
                    // Just error out for now
                    panic!("{}", "\nThere was an error in a critical step. Stopping execution.".red())
                }
            },
        }
    }
}

struct DayInfo {
    project_name: String,
    day_title: String,
    day_url: String,
    input_structure_name: String,
}

struct SetupStep {
    step_message: String,
    step_executor: fn(&DayInfo) -> io::Result<()>,
    critical: bool,
}

// Maybe split these up if we want to add a test case
// fn lookup_day_info(year: &str, day_num: i32) -> (String, String, String) {

// }

// fn get_day_title(year: &str, day_num: u8) -> String {
//     // 
// }

fn get_day_title_and_project_name(year: &str, day_num: u8, input_structure_name: String) -> DayInfo {
    // Determine the link for the day
    let day_url = format!("https://adventofcode.com/{year}/day/{day_num}");

    // Get the title of this day's question from the html response from day_url
    let day_html = reqwest::blocking::get(&day_url)
        .expect("Failed to get response for the page from this day's question...")
        .text()
        .expect("Failed to get the text from the page from this day's question");
    let title = {
        let start_title = day_html.find("--- ").expect("Couldn't find beginning of title") + 4;
        let end_title = day_html.find(" ---").expect("Couldn't find end of title");
        &day_html[start_title..end_title]
    };
    let day_title = format!("Advent of Code - {title}");

    // let h2_content = String::from("--- Day 6: Tuning Trouble ---");
    // let day_title = format!(
    //     "Advent of Code - {}",
    //     (&h2_content).replace("---", "").trim()
    // );

    // Get the project name from the day_title
    let project_name = format!(
        "day{}-{}",
        day_num,
        day_title
            .split_once(": ")
            .expect("Title for the day not in the 'Day <#>: <Day Title>' format we were expecting...")
            .1
            .split(' ')
            .map(|title_word| title_word.to_lowercase())
            .collect::<Vec<String>>()
            .join("-")
    );

    // Return the info for the day
    DayInfo {
        project_name,
        day_title,
        day_url,
        input_structure_name,
    }
}

fn get_file_bg_string(file_path: &Path) -> ColoredString {
    file_path.to_str().unwrap().on_truecolor(60, 70, 80)
}

fn print_already_exists(file_path: &Path) {
    println!(
        "{}",
        format!("  {} already exists, skipping...", get_file_bg_string(file_path)).yellow()
    );
}

fn print_created(file_path: &Path) {
    println!(
        "{}",
        format!("  {} created!", get_file_bg_string(file_path)).green()
    );
}

// TODO: logging?
// TODO: errors?
// TODO: possibly make custom errors (not just IO errors) that could be multiple types

// region: Advent Setup Steps

// Example step...
// fn list_files() {
//     // Test running ls
//     println!("Running 'ls':");
//     Command::new("ls").status().expect("ls command failed to start");
//     println!();
// }

/// Checks to see if a folder called `day_info.project_name` is in the current directory already and if not,
///  create a new rust project with that name using `cargo new` (with default values passed to it)
fn create_rust_project(day_info: &DayInfo) -> io::Result<()> {
    // Check if the folder exists in the current folder
    let project_path = Path::new(&day_info.project_name);
    if project_path.exists() {
        // TODO: we want to be able to add tempating things to existing ones so maybe don't error here...
        // ? Should I check for existence of cargo.toml?
        println!("{}", format!("  The project/directory '{}' already exists. Not creating a new one", day_info.project_name).yellow());
        return Ok(());
    }

    // TODO: maybe prompt user whether they wish to create a new project, initialize the existing folder or just exit the program

    // Run cargo new to create a new project
    Command::new("cargo")
        .args(["new", &day_info.project_name])
        .status()
        .expect("Failed to create new project with 'cargo new'");

    // Just return okay if it didn't already crash
    Ok(())
}

/// Add a gitignore if it doesn't exist already (creating a new cargo project doesn't add a gitignore if the project is already contained in a git repo)
fn add_gitignore(day_info: &DayInfo) -> io::Result<()> {
    // Get the path to the gitignore
    let gitignore_path = Path::new(&day_info.project_name).join(".gitignore");

    if gitignore_path.exists() {
        print_already_exists(&gitignore_path);
    }
    else {
        // Create the file (with '/target' rule setup in it)
        fs::write(&gitignore_path, "/target\n")?;
        print_created(&gitignore_path);
    }

    Ok(())
}

fn create_readme(day_info: &DayInfo) -> io::Result<()> {
    // Check if README already exists
    let readme_path = Path::new(&day_info.project_name).join("README.md");

    if readme_path.exists() {
        print_already_exists(&readme_path);
    }
    else {
        // Reference the url to the page for this day's question
        let day_url_note = format!("See the question for more details: {}", day_info.day_url);
    
        // Template out the README file
        let readme_contents = format!("# {}\n{day_url_note}\n", day_info.day_title);
    
        // Create the file
        fs::write(&readme_path, readme_contents)?;
        print_created(&readme_path);
    }

    // Return Ok if nothing else complained
    Ok(())
}

fn create_inputs(day_info: &DayInfo) -> io::Result<()> {
    // Helper function to create the input files
    fn create_input_files<'a>(inputs_dir_path: &Path, inputs_to_create: impl IntoIterator<Item = &'a str>) -> io::Result<()> {
        for filename in inputs_to_create {
            let input_filepath = inputs_dir_path.join(filename);
            if !input_filepath.exists() {
                File::create(&input_filepath)?;
                print_created(&input_filepath);
            }
            else {
                print_already_exists(&input_filepath);
            }
        }
        Ok(())
    }

    // Define the input files we want to create. At the moment just the two: "example" & "question"
    let inputs_to_create = ["example", "question"];

    // Check if inputs dir exists
    let inputs_path = Path::new(&day_info.project_name).join("inputs");
    if inputs_path.is_dir() {
        println!("{}", format!("  Directory '{}' already exists", inputs_path.display()).yellow());
        // If it does exist, check if example and question files exist, creating the files if not
        create_input_files(&inputs_path, inputs_to_create)?;
        Ok(())
    }
    else if inputs_path.is_file() {
        // This shouldn't happen but at the same time don't want to just delete it...
        // Maybe just fail this step with a note to retry running it after removing/renaming the file
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists, 
            "Found 'inputs' file already here. Remove it, rename it, or change it to a directory if you want to add the input files."
        ))
    }
    else {
        // If not, create it and create files
        println!("  Creating '{}' directory", inputs_path.display());

        // Create the inputs dir
        fs::create_dir(&inputs_path)?;

        // Create the input files
        create_input_files(&inputs_path, inputs_to_create)?;

        // Temp just return ok
        Ok(())
    }
}

fn template_main_rs(day_info: &DayInfo) -> io::Result<()> {
// fn template_main_rs(day_info: &DayInfo) -> Result<(), tera::Error> {
    // Load the template, including it into the binary
    println!("  Loading main.rs template file...");
    let mut tera_template = Tera::default();
    tera_template
        .add_raw_template(
            "main.rs", 
            include_str!("../templates/main.rs")
        )
        .expect("Failed to load template file...");
    // TODO: probably remove the panic causing things from this...

    // Set up the context for the template to be rendered in
    let mut context = Context::new();
    context.insert("input_structure_name", &day_info.input_structure_name);

    // Render the template
    println!("  Rendering out main.rs template...");
    let main_rs_contents = tera_template
        .render("main.rs", &context)
        .expect("Failed to render the main.rs template...");

    // TODO: Check if main.rs is different than default maybe? If it is, possibly prompt the user if it should be replaced
    // Write the template to the src/main.rs file
    let main_rs_path = Path::new(&day_info.project_name)
        .join("src")
        .join("main.rs");
    println!("  Overwriting {}...", get_file_bg_string(&main_rs_path));
    fs::write( main_rs_path, main_rs_contents)
        .expect("Failed to write main.rs");

    // TODO: figure out errors as this one doesn't really necessarily return io errors only
    Ok(())
}

// endregion