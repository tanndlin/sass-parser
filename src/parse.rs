use crate::types::{Block, Style, Token};

pub fn parse(tokens: Vec<Token>) -> Vec<Block> {
    Parser::new(tokens).parse()
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn parse(&mut self) -> Vec<Block> {
        let mut classes = Vec::new();

        while self.current_token() != Token::Eof {
            classes.push(self.parse_class());
        }

        classes
    }

    fn parse_class(&mut self) -> Block {
        let mut block = Block::new(self.parse_selector(), vec![], vec![]);

        self.expect(&Token::LBrace);

        while self.current_token() != Token::RBrace {
            let next_next_token = self.tokens[self.position + 1].clone();
            if next_next_token == Token::Colon {
                block.styles.push(self.parse_style());
            } else {
                block.sub_blocks.push(self.parse_class());
            }
        }

        self.expect(&Token::RBrace);
        block
    }

    fn parse_selector(&mut self) -> String {
        let mut selector = String::new();

        loop {
            if self.current_token().is_selector() || self.current_token().is_ident() {
                selector.push_str(self.current_token().to_string());
                self.position += 1;
            } else {
                break;
            }
        }

        selector
    }

    fn current_token(&self) -> Token {
        self.tokens[self.position].clone()
    }

    fn expect(&mut self, expected: &Token) {
        assert!(
            !(&self.current_token() != expected),
            "Expected {:?}, got {:?}",
            expected,
            self.current_token()
        );

        self.position += 1;
    }

    fn parse_style(&mut self) -> Style {
        let name = self.parse_ident();
        self.expect(&Token::Colon);
        let value = self.parse_ident();
        self.expect(&Token::SemiColon);
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
        assert_eq!(classes[0].sub_blocks.len(), 1);
        assert_eq!(classes[0].sub_blocks[0].selector, "&.child");
        assert_eq!(classes[0].sub_blocks[0].styles.len(), 1);
        assert_eq!(
            classes[0].sub_blocks[0].styles[0],
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

    #[test]
    fn parse_attribute_selector() {
        let tokens = vec![
            Token::Class,
            Token::Ident("parent".to_string()),
            Token::LBracket,
            Token::Ident("data-attr".to_string()),
            Token::Equals,
            Token::Ident("'value'".to_string()),
            Token::RBracket,
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
        assert_eq!(classes[0].selector, r".parent[data-attr='value']");
        assert_eq!(classes[0].styles.len(), 1);
        assert_eq!(
            classes[0].styles[0],
            Style {
                name: "color".to_string(),
                value: "red".to_string()
            }
        );
    }
}
