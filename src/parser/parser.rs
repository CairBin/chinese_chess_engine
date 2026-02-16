use crate::core::piece::Color;
use crate::parser::lexer::{Lexer, Token, Keywords};
use crate::parser::ast::ASTNode;

// 语法分析器
#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    // 创建新的语法分析器
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        
        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }
    
    // 前进到下一个标记
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
    
    // 期望当前标记是指定类型
    fn expect_token(&mut self, expected: Token) -> bool {
        if self.current_token == expected {
            self.next_token();
            true
        } else {
            false
        }
    }
    
    // 期望当前标记是指定关键字
    fn expect_keyword(&mut self, expected: Keywords) -> bool {
        match &self.current_token {
            Token::Keyword(keyword) => {
                if keyword == &expected {
                    self.next_token();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    
    // 解析颜色
    fn parse_color(&mut self) -> Option<Color> {
        match self.current_token {
            Token::Red => {
                self.next_token();
                Some(Color::Red)
            }
            Token::Black => {
                self.next_token();
                Some(Color::Black)
            }
            _ => None,
        }
    }
    
    // 解析数字
    fn parse_number(&mut self) -> Option<u32> {
        match self.current_token {
            Token::Number(n) => {
                let num = n;
                self.next_token();
                Some(num)
            }
            _ => None,
        }
    }
    
    // 解析标识符
    fn parse_identifier(&mut self) -> Option<String> {
        match self.current_token {
            Token::Identifier(ref s) => {
                let ident = s.clone();
                self.next_token();
                Some(ident)
            }
            _ => None,
        }
    }
    
    // 解析坐标
    fn parse_coordinate(&mut self) -> Option<(u8, u8)> {
        match self.current_token {
            Token::Coordinate(x, y) => {
                let coord = (x, y);
                self.next_token();
                Some(coord)
            }
            _ => None,
        }
    }
    
    // 解析创建游戏命令
    fn parse_create_game(&mut self) -> Option<ASTNode> {
        if !self.expect_keyword(Keywords::Create) {
            return None;
        }
        
        if !self.expect_keyword(Keywords::Game) {
            return None;
        }
        
        Some(ASTNode::CreateGame)
    }
    
    // 解析加入游戏命令
    fn parse_join_game(&mut self) -> Option<ASTNode> {
        if !self.expect_keyword(Keywords::Join) {
            return None;
        }
        
        if !self.expect_keyword(Keywords::Game) {
            return None;
        }
        
        let game_id = self.parse_number()?;
        let name = self.parse_identifier()?;
        let color = self.parse_color()?;
        
        Some(ASTNode::JoinGame {
            game_id,
            name,
            color,
        })
    }
    
    // 解析移动命令
    fn parse_move(&mut self) -> Option<ASTNode> {
        if !self.expect_keyword(Keywords::Game) {
            return None;
        }
        
        let game_id = self.parse_number()?;
        let color = self.parse_color()?;
        
        if !self.expect_keyword(Keywords::Move) {
            return None;
        }
        
        let (from_x, from_y) = self.parse_coordinate()?;
        
        if !self.expect_keyword(Keywords::To) {
            return None;
        }
        
        let (to_x, to_y) = self.parse_coordinate()?;
        
        Some(ASTNode::Move {
            game_id,
            color,
            from_x,
            from_y,
            to_x,
            to_y,
        })
    }
    
    // 解析悔棋命令
    fn parse_undo(&mut self) -> Option<ASTNode> {
        if !self.expect_keyword(Keywords::Undo) {
            return None;
        }
        
        if !self.expect_keyword(Keywords::Game) {
            return None;
        }
        
        let game_id = self.parse_number()?;
        
        Some(ASTNode::Undo {
            game_id,
        })
    }
    
    // 解析获取游戏状态命令
    fn parse_get_game(&mut self) -> Option<ASTNode> {
        if !self.expect_keyword(Keywords::Get) {
            return None;
        }
        
        if !self.expect_keyword(Keywords::Game) {
            return None;
        }
        
        let game_id = self.parse_number()?;
        
        Some(ASTNode::GetGame {
            game_id,
        })
    }
    
    // 解析命令
    pub fn parse(&mut self) -> ASTNode {
        match self.current_token {
            Token::Keyword(Keywords::Create) => {
                if let Some(node) = self.parse_create_game() {
                    node
                } else {
                    ASTNode::Invalid
                }
            }
            Token::Keyword(Keywords::Join) => {
                if let Some(node) = self.parse_join_game() {
                    node
                } else {
                    ASTNode::Invalid
                }
            }
            Token::Keyword(Keywords::Game) => {
                if let Some(node) = self.parse_move() {
                    node
                } else {
                    ASTNode::Invalid
                }
            }
            Token::Keyword(Keywords::Undo) => {
                if let Some(node) = self.parse_undo() {
                    node
                } else {
                    ASTNode::Invalid
                }
            }
            Token::Keyword(Keywords::Get) => {
                if let Some(node) = self.parse_get_game() {
                    node
                } else {
                    ASTNode::Invalid
                }
            }
            _ => ASTNode::Invalid,
        }
    }
}
