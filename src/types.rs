#[derive(Debug, PartialEq, Clone)]
pub struct Style {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Class {
    pub selector: String,
    pub styles: Vec<Style>,
    pub sub_classes: Vec<Class>,
}

pub struct ShallowClass<'a> {
    pub selector: &'a String,
    pub styles: &'a Vec<Style>,
}

impl Class {
    pub fn new(selector: String, styles: Vec<Style>, sub_classes: Vec<Class>) -> Class {
        Class {
            selector,
            styles,
            sub_classes,
        }
    }

    pub fn shallow(&self) -> (ShallowClass, &Vec<Class>) {
        (
            ShallowClass {
                selector: &self.selector,
                styles: &self.styles,
            },
            &self.sub_classes,
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    LBrace,
    RBrace,
    SemiColon,
    Colon,
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
    pub fn from_str(s: &str) -> Token {
        match s {
            "{" => Token::LBrace,
            "}" => Token::RBrace,
            ";" => Token::SemiColon,
            ":" => Token::Colon,
            "." => Token::Class,
            ">" => Token::DirectChild,
            "*" => Token::All,
            "," => Token::And,
            "+" => Token::After,
            "~" => Token::Before,
            "&" => Token::Root,
            _ => Token::Ident(s.to_string()),
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::SemiColon => ";",
            Token::Colon => ":",
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
        )
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, Token::Ident(_))
    }
}
