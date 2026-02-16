use std::collections::HashMap;
use super::piece::PieceType;

use super::board::Board;
use super::piece::{Color, Piece};

// 游戏状态枚举
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GameStatus{
    Playing,
    RedWon,
    BlackWon,
    Stalemate,
}

// 玩家结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Player{
    pub name: String,
    pub color: Color,
}

// 游戏结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Game{
    pub id: u32,
    pub board: Board,
    pub players: Vec<Player>,
    pub status: GameStatus,
    pub current_turn: Color,
    pub move_history: Vec<MoveRecord>,  // 用于保存历史操作记录
}


// 移动记录
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MoveRecord {
    from_x: u8,
    from_y: u8,
    to_x: u8,
    to_y: u8,
    moved_piece: Piece,
    captured_piece: Piece,
    hash_before: u64,
}


// 游戏管理者
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ruler{
    games: HashMap<u32, Game>,
    next_game_id: u32,
}

impl Ruler{
    pub fn new() -> Self{
        Ruler{
            games: HashMap::new(),
            next_game_id: 1,
        }
    }

    // 创建新游戏
    pub fn create_game(&mut self) -> u32{
        let game_id = self.next_game_id;
        self.next_game_id += 1;

        let game = Game{
            id: game_id,
            board: Board::new(),
            players: Vec::new(),
            status: GameStatus::Playing,
            current_turn: Color::Red, // 红方先行
            move_history: Vec::new(),
        };

        self.games.insert(game_id, game);
        game_id
    }

    // 获取游戏
    pub fn get_game(&self, game_id: u32) -> Option<&Game> {
        self.games.get(&game_id)
    }

    // 加入玩家
    pub fn add_player_to_game(&mut self, game_id: u32, name: String, color: Color) -> bool {
        if let Some(game) = self.games.get_mut(&game_id){
            // 检查游戏是否已经有两个玩家
            if game.players.len() >= 2{
                return false;
            }

            // 检查颜色是否已经被占用
            for player in &game.players{
                if player.color == color{
                    return false;
                }
            }

            // 添加玩家
            game.players.push(Player{name, color});
            true
        }else{
            false
        }
    }


    // 执行移动
    pub fn make_move(&mut self, game_id: u32, from_x: u8, from_y: u8, to_x: u8, to_y: u8) -> bool {
        if let Some(game) = self.games.get_mut(&game_id) {
            // 检查游戏是否正在进行
            if game.status != GameStatus::Playing {
                return false;
            }
            
            // 创建位置
            let from = super::position::Position::new(from_x, from_y);
            let to = super::position::Position::new(to_x, to_y);
            
            // 检查移动是否合法
            if !game.board.is_move_valid(from, to, game.current_turn) {
                return false;
            }
            
            // 检查移动后是否仍然被将军
            if !game.board.is_move_safe(from, to, game.current_turn) {
                return false;
            }

            // 记录移动前的状态
            let moved_piece = game.board.get_piece(from);
            let captured_piece = game.board.get_piece(to);
            let hash_before = game.board.get_hash();
            
            // 执行移动
            game.board.make_move(from, to);

            // 记录移动历史
            game.move_history.push(MoveRecord {
                from_x,
                from_y,
                to_x,
                to_y,
                moved_piece,
                captured_piece,
                hash_before,
            });

            if captured_piece.piece_type == PieceType::King {
                // 游戏结束，当前玩家获胜
                game.status = match game.current_turn {
                    Color::Red => GameStatus::RedWon,
                    Color::Black => GameStatus::BlackWon,
                    _ => GameStatus::Playing,
                };
            }
            
            // 检查游戏是否结束
            let opponent_color = match game.current_turn {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
                _ => return false,
            };
            
            if game.board.is_checkmate(opponent_color) {
                game.status = match game.current_turn {
                    Color::Red => GameStatus::RedWon,
                    Color::Black => GameStatus::BlackWon,
                    _ => GameStatus::Playing,
                };
            } else if game.board.is_stalemate(opponent_color) {
                game.status = GameStatus::Stalemate;
            }
            
            // 切换回合
            game.current_turn = opponent_color;
            
            true
        } else {
            false
        }
    }


    // 删除游戏
    pub fn remove_game(&mut self, game_id: u32) -> bool {
        self.games.remove(&game_id).is_some()
    }
    
    // 获取所有游戏
    pub fn get_all_games(&self) -> Vec<&Game> {
        self.games.values().collect()
    }

    // 悔棋
    pub fn undo_move(&mut self, game_id: u32) -> bool {
        if let Some(game) = self.games.get_mut(&game_id) {
            // 检查是否有移动历史可以悔棋
            if game.move_history.is_empty() {
                return false;
            }
            
            // 获取最后一步移动
            if let Some(last_move) = game.move_history.pop() {
                // 创建位置
                let from = super::position::Position::new(last_move.from_x, last_move.from_y);
                let to = super::position::Position::new(last_move.to_x, last_move.to_y);
                
                // 恢复棋盘状态
                // 将移动的棋子放回原来的位置
                game.board.set_piece(from, last_move.moved_piece);
                
                // 恢复被吃掉的棋子
                game.board.set_piece(to, last_move.captured_piece);
                
                // 切换回合
                game.current_turn = match game.current_turn {
                    Color::Red => Color::Black,
                    Color::Black => Color::Red,
                    _ => return false,
                };
                
                // 如果游戏之前是结束状态，恢复为进行中
                if game.status != GameStatus::Playing {
                    game.status = GameStatus::Playing;
                }
                
                return true;
            }
        }
        
        false
    }
}

