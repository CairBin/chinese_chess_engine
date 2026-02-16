
// 关键字
#[derive(Debug, Clone, PartialEq)]
pub enum Keywords{
    Game,
    Create,
    Join,
    Move,
    Undo,
    Get,
    To,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token{
    Keyword(Keywords),

    // 颜色
    Red,
    Black,

    // 数字
    Number(u32),

    // 坐标
    Coordinate(u8, u8),

    // 括号
    LeftParen, // (
    RightParen, // )

    // 逗号
    Comma,

    // 标识符
    Identifier(String),

    // 结束标记
    Eof, 
}

// 词法分析器
#[derive(Debug, Clone)]
pub struct Lexer{
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}


impl Lexer{
    pub fn new(input: &str) -> Self{
        let mut lexer = Lexer {
            input: input.trim().to_string(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }


    // 读取下一个字符
    fn read_char(&mut self) {
        if self.read_position >= self.input.len(){
            self.ch = '\0';
        }else{
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    // 跳过空白字符
    fn skip_whitespace(&mut self){
        while self.ch.is_whitespace(){
            self.read_char();
        }
    }

    // 读取数字
    fn read_number(&mut self) -> u32{
        let start = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[start..self.position].parse().unwrap()
    }


    // 读取标识符
    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.ch.is_alphanumeric() {
            self.read_char();
        }
        self.input[start..self.position].to_string()
    }
    
    // 读取坐标
    fn read_coordinate(&mut self) -> Option<(u8, u8)> {
        // 期望 '('
        if self.ch != '(' {
            return None;
        }
        self.read_char();
        
        // 读取 x 坐标
        let x = if self.ch.is_digit(10) {
            self.read_number() as u8
        } else {
            return None;
        };
        
        // 期望 ','
        if self.ch != ',' {
            return None;
        }
        self.read_char();
        
        // 读取 y 坐标
        let y = if self.ch.is_digit(10) {
            self.read_number() as u8
        } else {
            return None;
        };
        
        // 期望 ')'
        if self.ch != ')' {
            return None;
        }
        self.read_char();
        
        Some((x, y))
    }

    // 获取下一个标记
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let token = match self.ch {
            '(' => {
                // 检查是否是坐标
                if let Some((x, y)) = self.read_coordinate() {
                    Token::Coordinate(x, y)
                } else {
                    self.read_char();
                    Token::LeftParen
                }
            }
            ')' => {
                self.read_char();
                Token::RightParen
            }
            ',' => {
                self.read_char();
                Token::Comma
            }
            '0'..='9' => {
                Token::Number(self.read_number())
            }
            'a'..='z' | 'A'..='Z' => {
                let ident = self.read_identifier().to_uppercase();
                match ident.as_str() {
                    "GAME" => Token::Keyword(Keywords::Game),
                    "CREATE" => Token::Keyword(Keywords::Create),
                    "JOIN" => Token::Keyword(Keywords::Join),
                    "MOVE" => Token::Keyword(Keywords::Move),
                    "UNDO" => Token::Keyword(Keywords::Undo),
                    "GET" => Token::Keyword(Keywords::Get),
                    "TO" => Token::Keyword(Keywords::To),
                    "RED" => Token::Red,
                    "BLACK" => Token::Black,
                    _ => Token::Identifier(ident),
                }
            }
            '\0' => Token::Eof,
            _ => {
                self.read_char();
                Token::Eof
            }
        };
        
        token
    }

    // 生成所有标记
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token == Token::Eof {
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}