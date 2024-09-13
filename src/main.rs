mod lexer;
mod parse;
mod types;
mod utils;

fn main() {
    let input = parse::read_file("styles.scss");
    let mut l = lexer::Lexer::new(input);
    let tokens = l.get_tokens();

    println!("{:?}", tokens);
}
