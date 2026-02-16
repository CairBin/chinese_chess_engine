use super::position::Position;
use super::piece::{Piece, PieceType, Color};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bitboard{
    pieces: [u64; 8],   // [King Advisor Elephant Horse Chariot Cannon Pawn None]
    colors: [u64; 3],   // [None Red Black]
    palace: [u64; 2],   // [Red Black]
}


impl Bitboard{
    // 创建新的bit棋盘
    pub fn new() -> Self{
        Bitboard{
            pieces: [0; 8],
            colors: [0; 3],
            palace: [0; 2],
        }
    }


    // 初始化特殊位置(九宫格)
    pub fn init_special_positions(&mut self){
        // red
        for x in 3..=5{
            for y in 0..=2{
                let index = x + y*9;
                if index < 64{
                    self.palace[0] |= 1 << index;
                }
            }
        }

        // black
        for x in 3..=5 {
            for y in 7..=9 {
                let index = x + y * 9;
                if index < 64 {
                    self.palace[1] |= 1 << index;
                }
            }
        }
    }

    // 从坐标转换到位棋盘索引
    pub fn pos_to_index(pos: Position) -> usize {
        (pos.x() + pos.y() * 9) as usize
    }
    
    // 从位棋盘索引转换到坐标
    pub fn index_to_pos(index: usize) -> Position {
        Position::new((index % 9) as u8, (index / 9) as u8)
    }
    
    // 设置棋子
    pub fn set_piece(&mut self, pos: Position, piece: Piece) {
        let index = Self::pos_to_index(pos);
        if index < 64 {
            let bit = 1 << index;
            
            // 清除旧的棋子信息
            for i in 0..8 {
                self.pieces[i] &= !bit;
            }
            for i in 0..3 {
                self.colors[i] &= !bit;
            }
            
            // 设置新的棋子信息
            if piece.piece_type != PieceType::None {
                let piece_index = match piece.piece_type {
                    PieceType::King => 0,
                    PieceType::Advisor => 1,
                    PieceType::Elephant => 2,
                    PieceType::Horse => 3,
                    PieceType::Chariot => 4,
                    PieceType::Cannon => 5,
                    PieceType::Pawn => 6,
                    PieceType::None => 7,
                };
                self.pieces[piece_index] |= bit;
                
                let color_index = match piece.color {
                    Color::Red => 1,
                    Color::Black => 2,
                    Color::None => 0,
                };
                self.colors[color_index] |= bit;
            }
        }
    }
    
    // 检查位置是否有特定类型的棋子
    pub fn has_piece(&self, pos: Position, piece_type: PieceType) -> bool {
        let index = Self::pos_to_index(pos);
        if index < 64 {
            let bit = 1 << index;
            
            let piece_index = match piece_type {
                PieceType::King => 0,
                PieceType::Advisor => 1,
                PieceType::Elephant => 2,
                PieceType::Horse => 3,
                PieceType::Chariot => 4,
                PieceType::Cannon => 5,
                PieceType::Pawn => 6,
                PieceType::None => 7,
            };
            
            (self.pieces[piece_index] & bit) != 0
        } else {
            false
        }
    }
    
    // 检查位置是否有特定颜色的棋子
    pub fn has_color(&self, pos: Position, color: Color) -> bool {
        let index = Self::pos_to_index(pos);
        if index < 64 {
            let bit = 1 << index;
            
            let color_index = match color {
                Color::Red => 1,
                Color::Black => 2,
                Color::None => 0,
            };
            
            (self.colors[color_index] & bit) != 0
        } else {
            false
        }
    }
    
    // 检查位置是否在九宫中
    pub fn is_in_palace(&self, pos: Position, color: Color) -> bool {
        let index = Self::pos_to_index(pos);
        if index < 64 {
            let bit = 1 << index;
            
            let palace_index = match color {
                Color::Red => 0,
                Color::Black => 1,
                _ => return false,
            };
            
            (self.palace[palace_index] & bit) != 0
        } else {
            false
        }
    }
}