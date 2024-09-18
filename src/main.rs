mod compiler;
mod lexer;
mod parse;
mod types;
mod utils;

fn main() {
    let input = utils::read_file("stress.scss");
    let mut l = lexer::Lexer::new(input);
    let tokens = l.get_tokens();

    let mut parser = parse::Parser::new(tokens);
    let classes = parser.parse();

    let compiler = compiler::Compiler::new(classes);
    compiler.compile("styles.css");
}
