use crate::types::{Class, Style, Token};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Class> {
        let mut classes = Vec::new();

        while self.current_token() != Token::Eof {
            classes.push(self.parse_class());
        }

        classes
    }

    fn parse_class(&mut self) -> Class {
        let mut class = Class {
            selector: self.parse_selector(),
            styles: Vec::new(),
            sub_classes: Vec::new(),
        };

        self.expect(Token::LBrace);

        while self.current_token() != Token::RBrace {
            let next_next_token = self.tokens[self.position + 1].clone();
            if next_next_token == Token::Colon {
                class.styles.push(self.parse_style());
            } else {
                class.sub_classes.push(self.parse_class());
            }
        }

        self.expect(Token::RBrace);

        class
    }

    fn parse_selector(&mut self) -> String {
        let mut selector = String::new();

        loop {
            match self.current_token() {
                Token::Class => {
                    selector.push('.');
                    self.position += 1;
                    selector.push_str(&self.parse_ident());
                }
                Token::DirectChild => {
                    selector.push('>');
                    self.position += 1;
                }
                Token::Root => {
                    selector.push('&');
                    self.position += 1;
                }
                Token::Ident(_) => {
                    selector.push_str(&self.parse_ident());
                }
                _ => break,
            }
        }

        selector
    }

    fn current_token(&self) -> Token {
        self.tokens[self.position].clone()
    }

    fn expect(&mut self, expected: Token) {
        if self.current_token() != expected {
            panic!("Expected {:?}, got {:?}", expected, self.current_token());
        }

        self.position += 1;
    }

    fn parse_style(&mut self) -> Style {
        let name = self.parse_ident();
        self.expect(Token::Colon);
        let value = self.parse_ident();
        self.expect(Token::SemiColon);
        Style { name, value }
    }

    fn parse_ident(&mut self) -> String {
        match self.current_token() {
            Token::Ident(s) => {
                self.position += 1;
                s
            }
            _ => panic!("Expected ident, got {:?}", self.current_token()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let tokens = vec![Token::Eof];
        let mut parser = Parser::new(tokens);
        let classes = parser.parse();
        assert!(classes.is_empty());
    }

    #[test]
    fn test_parse_single_class() {
        let tokens = vec![
            Token::Class,
            Token::Ident("example".to_string()),
            Token::LBrace,
            Token::Ident("color".to_string()),
            Token::Colon,
            Token::Ident("red".to_string()),
            Token::SemiColon,
            Token::RBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let classes = parser.parse();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].selector, ".example");
        assert_eq!(classes[0].styles.len(), 1);
        assert_eq!(
            classes[0].styles[0],
            Style {
                name: "color".to_string(),
                value: "red".to_string()
            }
        );
    }

    #[test]
    fn test_parse_nested_classes() {
        let tokens = vec![
            Token::Class,
            Token::Ident("parent".to_string()),
            Token::LBrace,
            Token::Root,
            Token::Class,
            Token::Ident("child".to_string()),
            Token::LBrace,
            Token::Ident("color".to_string()),
            Token::Colon,
            Token::Ident("blue".to_string()),
            Token::SemiColon,
            Token::RBrace,
            Token::RBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let classes = parser.parse();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].selector, ".parent");
        assert_eq!(classes[0].sub_classes.len(), 1);
        assert_eq!(classes[0].sub_classes[0].selector, ".child");
        assert_eq!(classes[0].sub_classes[0].styles.len(), 1);
        assert_eq!(
            classes[0].sub_classes[0].styles[0],
            Style {
                name: "color".to_string(),
                value: "blue".to_string()
            }
        );
    }

    #[test]
    fn test_parse_direct_child_selector() {
        let tokens = vec![
            Token::Class,
            Token::Ident("parent".to_string()),
            Token::DirectChild,
            Token::Class,
            Token::Ident("child".to_string()),
            Token::LBrace,
            Token::Ident("margin".to_string()),
            Token::Colon,
            Token::Ident("10px".to_string()),
            Token::SemiColon,
            Token::RBrace,
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let classes = parser.parse();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].selector, ".parent>.child");
        assert_eq!(classes[0].styles.len(), 1);
        assert_eq!(
            classes[0].styles[0],
            Style {
                name: "margin".to_string(),
                value: "10px".to_string()
            }
        );
    }
}
