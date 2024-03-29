#[derive(Clone, Copy)]
pub enum TokenTyp {
    EOF, // End of file
    SYMICOLON,
    
    //types
    IDENTIFIER,
    NUMBER,
    STRING,

    //formating stuff
    DOT, COMMA, UnderScore, 
    COLON, ColonColon,
    RArrow,

    //Math
    ADD, MIN, MUL, DIV, POW,
    AddAdd, MinMin,

    //vars
    LET, CONST,
    EQUAL, EqualEqual,
    AddEqual, MinEqual, MulEqual, DivEqual,

    //functions
    FN,
    LeftBracet, RightBracet,
    LeftParam, RightParam,
    Return,

}

impl TokenTyp {
    pub fn _str(&self) -> String {
        let str: &str = match *self {
            Self::EqualEqual => "equal equal (==)",
            Self::EQUAL => "equal (=)",
            Self::EOF => "eof",
            Self::NUMBER => "number",
            Self::ADD => "add",
            Self::MIN => "min",
            Self::MUL => "mul",
            Self::DIV => "div",
            Self::POW => "pow (**)",
            Self::AddAdd => "add add (++)",
            Self::MinMin => "min min (--)",
            Self::LET => "let",
            Self::CONST => "const",
            Self::AddEqual => "add equal (+=)",
            Self::MinEqual => "min equal (-=)",
            Self::DivEqual => "div equal (/=)",
            Self::FN => "fn",
            Self::LeftBracet => "left bracet ({)",
            Self::RightBracet => "right bracet (})",
            Self::LeftParam => "left param (()",
            Self::RightParam => "right param ())",
            Self::Return => "return",
            Self::DOT => "dot (.)",
            Self::COMMA => "comma, (,)",
            Self::UnderScore => "under score (_)",
            Self::STRING => "string",
            Self::IDENTIFIER => "identifer",
            Self::COLON => "colon (:)",
            Self::ColonColon => "colon colon (::)",
            Self::RArrow => "right arrow (->)",
             _ => "",
        };

        format!("{}", str)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenTyp,
    pub identifier: String,
    pub line: i32,
    pub line_str: String,

    pub file: String,
}

impl Token {
    pub fn new(_token_type: TokenTyp, line: i32, _ident: String, _file: String, _line_str: String) -> Self {
        Token { 
            token_type: _token_type,
            identifier: _ident.clone(),
            line: line.clone(),
            file: _file.clone(),
            line_str: _line_str,
        }
    }

    pub fn print(&self) {
        println!("{}:{} token {}:{}", self.file, self.line, self.token_type._str(), self.identifier);
    }
}