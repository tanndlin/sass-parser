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

    Root,
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::LBrace => "{".to_string(),
            Token::RBrace => "}".to_string(),
            Token::SemiColon => ";".to_string(),
            Token::Colon => ":".to_string(),
            Token::Eof => "EOF".to_string(),
            Token::Ident(s) => s.to_string(),
            Token::Class => ".".to_string(),
            Token::DirectChild => ">".to_string(),
            Token::Root => "&".to_string(),
        }
    }
}
