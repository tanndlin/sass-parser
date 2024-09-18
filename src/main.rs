mod compiler;
mod lexer;
mod parse;
mod types;
mod utils;

fn main() {
    let input = utils::read_file("styles.scss");
    let mut l = lexer::Lexer::new(input);
    let tokens = l.get_tokens();
    println!("{:?}", tokens);

    let mut parser = parse::Parser::new(tokens);
    let classes = parser.parse();
    println!("{:?}", classes);

    let compiler = compiler::Compiler::new(classes);
    compiler.compile("styles.css");
}
