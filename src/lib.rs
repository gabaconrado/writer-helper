#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub struct Config {
    filename: String,
    search_word: Option<String>,
    verbose: bool,
}

pub struct ParsingResult {
    pub line_count: u32,
    pub word_count: u32,
    pub section_count: u32,
    pub searched_words: HashMap<String, Vec<(u32, usize)>>,
}

impl Config {
    pub fn new() -> Config {
        let matches = setup_cli().get_matches();

        // Check verbose mode
        let verbose = matches.is_present("verbose");

        // Check word flag
        let search_word = if matches.is_present("word") {
            Some(String::from(matches.value_of("word").unwrap()))
        } else {
            None
        };

        // Get the filename from arguments
        let filename = String::from(matches.value_of("input-file").unwrap());

        Config {
            filename,
            search_word,
            verbose,
        }
    }
}

fn setup_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about("Simple assistent to assess your writings!")
        .author(crate_authors!())
        .arg(
            Arg::with_name("input-file")
                .help("Input file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("word")
                .help("Gather information about a specific word")
                .short("w")
                .long("word")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Set the verbose mode")
                .short("v")
                .long("verbose"),
        )
}

pub fn run(config: Config) -> Result<ParsingResult, Box<dyn Error>> {
    let mut line_count = 0;
    let mut word_count = 0;
    let mut section_count = 0;
    let mut searched_words = HashMap::new();

    // Read the file contents
    let content = fs::read_to_string(config.filename)?;

    // Setup the specific search words
    if let Some(word) = config.search_word {
        searched_words.insert(word, Vec::new());
    }

    if config.verbose {
        // Print the file content on screen
        println!("File content:\n{}", content);
    }

    // Iterate through the content
    for line in content.lines() {
        // Increment line count
        line_count += 1;
        // Check if it is a section and add to the counter
        if line.chars().nth(0) == Some('#') {
            section_count += 1;
            if config.verbose {
                println!("Section = {}", line);
            }
        } else {
            // Count the words for that line
            for (index, word) in line
                .split(|x| (x == ' ') || (x == '.') || (x == ','))
                .enumerate()
            {
                if !word.is_empty() {
                    word_count += 1;
                    if config.verbose {
                        println!("Word = {}", word);
                    }
                    if let Some(appearence) = searched_words.get(word) {
                        let mut new_vector = appearence.to_vec();
                        new_vector.push((line_count, index + 1));
                        searched_words.insert(String::from(word), new_vector);
                    }
                }
            }
        }
    }

    Ok(ParsingResult {
        line_count,
        word_count,
        section_count,
        searched_words,
    })
}
