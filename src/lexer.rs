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
        if self.position >= self.input.len() {
            return Token::Eof;
        }

        let whitespace = ['\n', '\r', '\t', ' '];
        while whitespace.contains(&self.ch) {
            self.read_char();
        }

        let tok = match self.ch {
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ';' => Token::SemiColon,
            ':' => Token::Colon,
            '\0' => Token::Eof,

            // Selectors
            '.' => Token::Class,
            '>' => Token::DirectChild,
            '*' => Token::All,
            ',' => Token::And,
            '+' => Token::After,
            '~' => Token::Before,

            '&' => Token::Root,

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_char_tokens() {
        let input = "{};:";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::LBrace,
            Token::RBrace,
            Token::SemiColon,
            Token::Colon,
            Token::Eof,
        ];

        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_selectors() {
        let input = ". > ";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![Token::Class, Token::DirectChild, Token::Eof];

        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_identifiers() {
        let input = "hello world";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::Ident("hello".to_string()),
            Token::Ident("world".to_string()),
            Token::Eof,
        ];

        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_mixed_input() {
        let input = ".class { color: red; }";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::Class,
            Token::Ident("class".to_string()),
            Token::LBrace,
            Token::Ident("color".to_string()),
            Token::Colon,
            Token::Ident("red".to_string()),
            Token::SemiColon,
            Token::RBrace,
            Token::Eof,
        ];

        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }
}
