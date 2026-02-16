use rand::Rng;
use super::board::Board;
use super::position::Position;
use super::piece::{Piece, PieceType};

// Zobrist Hash Table实现
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZobristHash{
    // 随机数表：[piece_type][color][x][y]
    table: [[[[u64; 10]; 9]; 3]; 8],
}

impl ZobristHash{
    // 创建新的Zobrist Hash表
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut table = [[[[0u64; 10]; 9]; 3]; 8];
        
        // 为每种棋子类型、颜色、位置生成随机数
        for pt in 0..8 {
            for c in 0..3 {
                for x in 0..9 {
                    for y in 0..10 {
                        table[pt][c][x][y] = rng.r#gen();
                    }
                }
            }
        }
        
        ZobristHash { table }
    }

    // 计算整个棋盘的哈希值
    pub fn calculate(&self, board: &Board) -> u64{
        let mut hash:u64 = 0;

        for x in 0..9{
            for y in 0..10{
                let pos = Position::new(x, y);
                let piece = board.get_piece(pos);

                if piece.piece_type != PieceType::None{
                    let pt = piece.piece_type.from_piece_type_to_usize();

                    let c = piece.color.from_color_to_usize();
                    
                    hash ^= self.table[pt][c][x as usize][y as usize];
                }
            }
        }

        hash

    }

    pub fn update(&self, old_hash:u64, from: Position, to: Position, moved_piece: Piece, captured_piece: Piece) -> u64{
        let mut hash = old_hash;

        // 移除移动的棋子
        let pt = moved_piece.piece_type.from_piece_type_to_usize();
        let c = moved_piece.color.from_color_to_usize();

        hash ^= self.table[pt][c][from.x() as usize][from.y() as usize];

        // 移除被吃的棋子
        if captured_piece.piece_type != PieceType::None{
            let captured_pt = captured_piece.piece_type.from_piece_type_to_usize();
            let captured_c = captured_piece.color.from_color_to_usize();

            hash ^= self.table[captured_pt][captured_c][to.x() as usize][to.y() as usize];
        }

        // 在新位置添加移动的棋子
        hash ^= self.table[pt][c][to.x() as usize][to.y() as usize];
        
        hash
    }

}