#[derive(Debug)]
pub enum TokenType {
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    BANG,
    BANGEQUAL,
    EQUAL,
    EQEQUAL,
    GT,
    GTEQUAL,
    LESS,
    LESSEQUAL,
    IDENTIFIER,
    STRING,
    NUMBER,
    AND,
    CLASS,
    ELSE,
    FALSE,
    RUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TokenType::LPAREN => write!(f, "("),
            TokenType::RPAREN => write!(f, ")"),
            TokenType::LBRACE => write!(f, "{{"),
            TokenType::RBRACE => write!(f, "}}"),
            _ => write!(f, "Later"), //TODO: improve this
        }
    }
}