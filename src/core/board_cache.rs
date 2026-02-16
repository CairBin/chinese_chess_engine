use std::collections::HashMap;
use super::position::Position;
use super::piece::Color;


// 棋盘缓存
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BoardCache{
    // 缓存棋子移动的合法目标位置
    // key: (from_pos, color), value: Vec<Position>
    legal_moves: HashMap<(Position, Color), Vec<Position>>,

    // 缓存将军状态
    // key: color, value: bool
    check_status: HashMap<Color, bool>,

    // 缓存当前哈希值
    current_hash: Option<u64>,
}

impl BoardCache{
    // 创建新的缓存
    pub fn new() -> Self{
        BoardCache{
            legal_moves: HashMap::new(),
            check_status: HashMap::new(),
            current_hash: None,
        }
    }

    // 缓存合法移动
    pub fn cache_legal_moves(&mut self, from_pos: Position, color: Color, moves: Vec<Position>) {
        self.legal_moves.insert((from_pos, color), moves);
    }

    // 获取缓存的合法移动
    pub fn get_cached_legal_moves(&self, from: Position, color: Color) -> Option<&Vec<Position>>{
        self.legal_moves.get(&(from, color))
    }

    // 缓存将军状态
    pub fn cache_check_status(&mut self, color: Color, status: bool){
        self.check_status.insert(color, status);
    }

    // 获取缓存的将军状态
    pub fn get_cached_check_status(&self, color: Color) -> Option<bool> {
        self.check_status.get(&color).copied()
    }

    // 缓存哈希值
    pub fn cache_hash(&mut self, hash: u64) {
        self.current_hash = Some(hash)
    }

    // 获取缓存的哈希值
    pub fn get_cached_hash(&self) -> Option<u64> {
        self.current_hash
    }

    // 清除缓存
    pub fn clear(&mut self){
        self.legal_moves.clear();
        self.check_status.clear();
        self.current_hash = None;
    }
}
