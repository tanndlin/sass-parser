use crate::types::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();

        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let whitespace = [' ', '\n', '\r', '\t'];
        while whitespace.contains(&self.ch) {
            self.read_char();
        }

        let tok = match self.ch {
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ';' => Token::SemiColon,
            ':' => Token::Colon,
            '\0' => Token::Eof,
            _ => Token::Ident(self.read_identifier()),
        };

        // Only read char if not ident
        match tok {
            Token::Ident(_) => {}
            _ => self.read_char(),
        }

        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            tokens.push(tok.clone());
            if tok == Token::Eof {
                break;
            }
        }
        tokens
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
