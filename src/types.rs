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
