use whelper::{Config, ParsingResult};

fn main() {
    let config = Config::new();

    match whelper::run(config) {
        Ok(parsing_result) => print_result(parsing_result),
        Err(e) => {
            eprintln!("Error running application: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_result(parsing_result: ParsingResult) {
    println!(
        "Process finished!\nLine count: {}\nWord count: {}\nSections count: {}",
        parsing_result.line_count, parsing_result.word_count, parsing_result.section_count
    );

    for (word, positions) in parsing_result.searched_words.into_iter() {
        println!("Specific word \"{}\" information:", word);
        for position in positions {
            println!("\tLine {}, word number {}", position.0, position.1);
        }
    }
}
