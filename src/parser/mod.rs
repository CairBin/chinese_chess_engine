pub mod lexer;
pub mod ast;
pub mod parser;


use crate::core::piece::Color;

// 命令类型枚举
#[derive(Debug, Clone)]
pub enum Command {
    // 移动命令: GAME <id> <color> MOVE (<from_x>,<from_y>) to (<to_x>,<to_y>)
    Move {
        game_id: u32,
        color: Color,
        from_x: u8,
        from_y: u8,
        to_x: u8,
        to_y: u8,
    },
    // 创建游戏命令: CREATE GAME
    CreateGame,
    // 加入游戏命令: JOIN GAME <id> <name> <color>
    JoinGame {
        game_id: u32,
        name: String,
        color: Color,
    },
    // 获取游戏状态命令: GET GAME <id>
    GetGame {
        game_id: u32,
    },
    // 悔棋命令: UNDO GAME <id>
    Undo {
        game_id: u32,
    },
    // 无效命令
    Invalid,
}

// 命令解析器
#[derive(Debug, Clone)]
pub struct CommandParser;

impl CommandParser {
    // 创建新的命令解析器
    pub fn new() -> Self {
        CommandParser
    }
    
    // 解析命令
    pub fn parse(&self, input: &str) -> Command {
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return Command::Invalid;
        }
        
        match parts[0].to_uppercase().as_str() {
            "GAME" => self.parse_move_command(&parts),
            "CREATE" => self.parse_create_game_command(&parts),
            "JOIN" => self.parse_join_game_command(&parts),
            "GET" => self.parse_get_game_command(&parts),
            "UNDO" => self.parse_undo_command(&parts),
            _ => Command::Invalid,
        }
    }
    
    // 解析移动命令
    fn parse_move_command(&self, parts: &[&str]) -> Command {
        if parts.len() < 9 {
            return Command::Invalid;
        }
        
        // 解析游戏ID
        let game_id = match parts[1].parse::<u32>() {
            Ok(id) => id,
            Err(_) => return Command::Invalid,
        };
        
        // 解析颜色
        let color = match parts[2].to_uppercase().as_str() {
            "RED" => Color::Red,
            "BLACK" => Color::Black,
            _ => return Command::Invalid,
        };
        
        // 检查是否是MOVE命令
        if parts[3].to_uppercase().as_str() != "MOVE" {
            return Command::Invalid;
        }
        
        // 解析起始位置
        let from_pos = &parts[4];
        let from_coords = self.parse_coordinates(from_pos);
        let (from_x, from_y) = match from_coords {
            Some((x, y)) => (x, y),
            None => return Command::Invalid,
        };
        
        // 检查是否是TO
        if parts[5].to_uppercase().as_str() != "TO" {
            return Command::Invalid;
        }
        
        // 解析目标位置
        let to_pos = &parts[6];
        let to_coords = self.parse_coordinates(to_pos);
        let (to_x, to_y) = match to_coords {
            Some((x, y)) => (x, y),
            None => return Command::Invalid,
        };
        
        Command::Move {
            game_id,
            color,
            from_x,
            from_y,
            to_x,
            to_y,
        }
    }
    
    // 解析创建游戏命令
    fn parse_create_game_command(&self, parts: &[&str]) -> Command {
        if parts.len() != 2 || parts[1].to_uppercase().as_str() != "GAME" {
            return Command::Invalid;
        }
        
        Command::CreateGame
    }
    
    // 解析加入游戏命令
    fn parse_join_game_command(&self, parts: &[&str]) -> Command {
        if parts.len() != 5 || parts[1].to_uppercase().as_str() != "GAME" {
            return Command::Invalid;
        }
        
        // 解析游戏ID
        let game_id = match parts[2].parse::<u32>() {
            Ok(id) => id,
            Err(_) => return Command::Invalid,
        };
        
        // 解析玩家名称
        let name = parts[3].to_string();
        
        // 解析颜色
        let color = match parts[4].to_uppercase().as_str() {
            "RED" => Color::Red,
            "BLACK" => Color::Black,
            _ => return Command::Invalid,
        };
        
        Command::JoinGame {
            game_id,
            name,
            color,
        }
    }
    
    // 解析获取游戏状态命令
    fn parse_get_game_command(&self, parts: &[&str]) -> Command {
        if parts.len() != 3 || parts[1].to_uppercase().as_str() != "GAME" {
            return Command::Invalid;
        }
        
        // 解析游戏ID
        let game_id = match parts[2].parse::<u32>() {
            Ok(id) => id,
            Err(_) => return Command::Invalid,
        };
        
        Command::GetGame {
            game_id,
        }
    }
    
    // 解析悔棋命令
    fn parse_undo_command(&self, parts: &[&str]) -> Command {
        if parts.len() != 3 || parts[1].to_uppercase().as_str() != "GAME" {
            return Command::Invalid;
        }
        
        // 解析游戏ID
        let game_id = match parts[2].parse::<u32>() {
            Ok(id) => id,
            Err(_) => return Command::Invalid,
        };
        
        Command::Undo {
            game_id,
        }
    }
    
    // 解析坐标，格式为 (x,y)
    fn parse_coordinates(&self, input: &str) -> Option<(u8, u8)> {
        // 移除括号
        let input = input.trim_matches(|c| c == '(' || c == ')');
        
        // 分割坐标
        let coords: Vec<&str> = input.split(',').collect();
        if coords.len() != 2 {
            return None;
        }
        
        // 解析x和y
        let x = match coords[0].parse::<u8>() {
            Ok(x) => x,
            Err(_) => return None,
        };
        
        let y = match coords[1].parse::<u8>() {
            Ok(y) => y,
            Err(_) => return None,
        };
        
        Some((x, y))
    }
}
