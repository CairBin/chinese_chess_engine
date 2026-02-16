use crate::core::piece::Color;

// 抽象语法树节点
#[derive(Debug, Clone)]
pub enum ASTNode {
    // 创建游戏
    CreateGame,
    
    // 加入游戏
    JoinGame {
        game_id: u32,
        name: String,
        color: Color,
    },
    
    // 移动
    Move {
        game_id: u32,
        color: Color,
        from_x: u8,
        from_y: u8,
        to_x: u8,
        to_y: u8,
    },
    
    // 悔棋
    Undo {
        game_id: u32,
    },
    
    // 获取游戏状态
    GetGame {
        game_id: u32,
    },
    
    // 无效命令
    Invalid,
}