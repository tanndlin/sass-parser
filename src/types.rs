#[derive(Debug, PartialEq, Clone)]
pub struct Style {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub selector: String,
    pub styles: Vec<Style>,
    pub sub_blocks: Vec<Block>,
}

impl Block {
    pub fn new(selector: String, styles: Vec<Style>, sub_blocks: Vec<Block>) -> Block {
        Block {
            selector,
            styles,
            sub_blocks,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    LBrace,
    RBrace,
    SemiColon,
    Colon,
    LBracket,
    RBracket,
    Equals,
    Eof,
    Ident(String),

    // Selectors
    Class,
    DirectChild,
    All,
    And,
    After,
    Before,

    Root,
}

impl Token {
    #[allow(dead_code)]
    pub fn from_str(s: char) -> Option<Token> {
        match s {
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            ';' => Some(Token::SemiColon),
            ':' => Some(Token::Colon),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            '=' => Some(Token::Equals),
            '.' => Some(Token::Class),
            '>' => Some(Token::DirectChild),
            '*' => Some(Token::All),
            ',' => Some(Token::And),
            '+' => Some(Token::After),
            '~' => Some(Token::Before),
            '&' => Some(Token::Root),
            '\0' => Some(Token::Eof),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::SemiColon => ";",
            Token::Colon => ":",
            Token::LBracket => "[",
            Token::RBracket => "]",
            Token::Equals => "=",
            Token::Eof => "EOF",
            Token::Ident(s) => s,
            Token::Class => ".",
            Token::DirectChild => ">",
            Token::All => "*",
            Token::And => ",",
            Token::After => "+",
            Token::Before => "~",
            Token::Root => "&",
        }
    }

    pub fn is_selector(&self) -> bool {
        matches!(
            self,
            Token::Class
                | Token::DirectChild
                | Token::All
                | Token::And
                | Token::After
                | Token::Before
                | Token::Root
                | Token::Colon
                | Token::LBracket
                | Token::RBracket
                | Token::Equals
        )
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, Token::Ident(_))
    }
}
