use crate::types::Token;

pub struct Lexer {
    chars: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            chars: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();

        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.chars.len() {
            self.ch = '\0';
        } else {
            self.ch = self.chars[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        if self.position >= self.chars.len() {
            return Token::Eof;
        }

        let whitespace = ['\n', '\r', '\t', ' '];
        while whitespace.contains(&self.ch) {
            self.read_char();
        }

        let tok = match Token::from_str(self.ch) {
            Some(token) => token,
            None => {
                let ident = self.read_identifier();
                Token::Ident(ident)
            }
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
        // Check for a string
        if self.ch == '"' {
            self.read_char();
            while self.ch != '"' {
                self.read_char();
            }
            self.read_char();
            return self.chars[position..self.position].iter().collect();
        }

        if self.ch == '\'' {
            self.read_char();
            while self.ch != '\'' {
                self.read_char();
            }
            self.read_char();
            return self.chars[position..self.position].iter().collect();
        }

        while is_valid_ident_char(self.ch) {
            self.read_char();
        }

        self.chars[position..self.position].iter().collect()
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok == Token::Eof {
                tokens.push(tok);
                break;
            }

            tokens.push(tok);
        }
        tokens
    }
}

fn is_valid_ident_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '-'
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

    #[test]
    fn lex_attribute_selector_1() {
        let input = "input[type=number]";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::Ident("input".to_string()),
            Token::LBracket,
            Token::Ident("type".to_string()),
            Token::Equals,
            Token::Ident("number".to_string()),
            Token::RBracket,
            Token::Eof,
        ];

        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn lex_attribute_selector_2() {
        let input = "[data-attr='value']";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::LBracket,
            Token::Ident("data-attr".to_string()),
            Token::Equals,
            Token::Ident("'value'".to_string()),
            Token::RBracket,
            Token::Eof,
        ];

        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn lex_basic_styles() {
        let input = ".class { color: red; margin: 10px; padding: 0; }";
        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::Class,
            Token::Ident("class".to_string()),
            Token::LBrace,
            Token::Ident("color".to_string()),
            Token::Colon,
            Token::Ident("red".to_string()),
            Token::SemiColon,
            Token::Ident("margin".to_string()),
            Token::Colon,
            Token::Ident("10px".to_string()),
            Token::SemiColon,
            Token::Ident("padding".to_string()),
            Token::Colon,
            Token::Ident("0".to_string()),
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
