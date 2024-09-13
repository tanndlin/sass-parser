use crate::types::*;

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
            class.styles.push(self.parse_style());
        }

        self.expect(Token::RBrace);

        class
    }

    fn parse_selector(&mut self) -> String {
        match self.current_token() {
            Token::Ident(s) => {
                self.position += 1;
                s
            }
            _ => panic!("Expected ident, got {:?}", self.current_token()),
        }
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
