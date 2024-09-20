#![deny(clippy::all)]

mod compiler;
mod lexer;
mod parse;
mod types;
mod utils;

fn main() {
    // Take filename from args
    let args: Vec<String> = std::env::args().collect();
    let (file_name, result_file_name) = utils::parse_args(args);

    let input = utils::read_file(file_name.as_str());
    let start = std::time::Instant::now();

    let tokens = lexer::tokenize(input);
    let classes = parse::parse(tokens);
    let css = compiler::compile(classes);

    let duration = start.elapsed();
    println!("Time elapsed in parsing and compiling: {:?}", duration);

    std::fs::write(result_file_name, css).expect("Unable to write file");
}
