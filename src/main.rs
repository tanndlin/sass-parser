mod compiler;
mod lexer;
mod parse;
mod types;
mod utils;

fn main() {
    // Take filename from args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    if !file_name.ends_with(".scss") {
        eprintln!("Expected a .scss file, got {}", file_name);
        std::process::exit(1);
    }

    let result_file_name = file_name.replace(".scss", ".css");

    let input = utils::read_file(file_name);
    let mut l = lexer::Lexer::new(input);
    let tokens = l.get_tokens();

    let mut parser = parse::Parser::new(tokens);
    let classes = parser.parse();

    let compiler = compiler::Compiler::new(classes);
    compiler.compile(&result_file_name);
}
