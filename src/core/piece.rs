use std::hash::Hash;

// 棋子类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PieceType {
    King,      // 将/帅
    Advisor,   // 士/仕
    Elephant,  // 象/相
    Horse,     // 马/马
    Chariot,   // 车/车
    Cannon,    // 炮/炮
    Pawn,      // 卒/兵
    None,      // 空
}

// 颜色枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Color {
    Red,
    Black,
    None,
}

// 棋子结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Default for Piece {
    fn default() -> Self {
        Piece {
            piece_type: PieceType::None,
            color: Color::None,
        }
    }
}


impl PieceType{
    pub fn from_piece_type_to_usize(&self) -> usize{
        match self{
            PieceType::King => 0,
            PieceType::Advisor => 1,
            PieceType::Elephant =>2,
            PieceType::Horse => 3,
            PieceType::Chariot => 4,
            PieceType::Cannon => 5,
            PieceType::Pawn => 6,
            PieceType::None => 7,
        }
    }
}

impl Color{
    pub fn from_color_to_usize(&self) -> usize{
        match self{
            Color::Red => 1,
            Color::Black => 2,
            Color::None => 0,
        }
    }
}