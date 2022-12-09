# Adventemplate
I've noticed that the boilerplate code that I've been setting up each day for advent of code is very similar. To reduce the manual effort of creating it each day, I thought it might be convenient to have a simple script that generates some boilerplate code that so far seems to be in each solution. Past that, it also seemed like a fun excersise to learn some templating/code generation in Rust :)

## Goals for this script...
- Initially at least template the `main.rs` file
- Possibly also have it run `cargo new` and create inputs folder as well as a gitignore if it doesn't exist
- Maybe create a README linking to the question for the day
- Maybe add some command line params that can control which of these feature to do
  - say I want to just add READMEs to existing days that I have already solved but didn't add READMEs to yet
- Could be cool to add the release build of the program to the top level of the repo
- Maybe also look into the convenient error handling library that I've heard about

### Contents of the `main.rs` file
- Main function
  - Read in the question input
  - Do part one and part two on input, printing the result (possibly template in the message for the given day?)
- Helpers region
  - probably just create the region as we don't know what will be in it
  - possibly there will always be the input parsing function in there - it seems like there's always at least shared functionality there
- Part One/Two regions
  - Test case
    - Read in input
    - Get result from  

### Params
Steps we can choose to do or not:
  - cargo new (needs an input file)
    - possibly have it check the folder path and if it exists, have it prompt you for whether you want it to create a rust project at that folder - maybe error if the path is more than one level past an existing folder or maybe just make it only able to create in the current folder (this seems simpler)
  - template main file
  - add inputs dir
  - add gitignore
  - add README

### Usage...
