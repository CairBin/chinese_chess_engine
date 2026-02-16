use super::position::Position;
use super::piece::{Piece, PieceType, Color};
use super::bitboard::Bitboard;
use super::zobrist::ZobristHash;
use super::board_cache::BoardCache;

// 棋盘
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Board{
    // 9 x 10 棋盘
    pieces: [[Piece; 10]; 9],

    // 预算王的位置
    red_king_pos: Option<Position>,
    black_king_pos: Option<Position>,

    // zobrist hash
    zobrist_hash: ZobristHash,
    current_hash: u64,

    // bitboard
    bitboard: Bitboard,

    // 缓存机制
    cache: BoardCache,
}


impl Default for Board{
    fn default() -> Self{
        let mut board = Board{
            pieces: [[Piece::default(); 10]; 9],
            red_king_pos: None,
            black_king_pos: None,
            zobrist_hash: ZobristHash::new(),
            current_hash: 0,
            bitboard: Bitboard::new(),
            cache: BoardCache::new(),
        };

        board.bitboard.init_special_positions();
        board
    }
}

impl Board{
    pub fn new() -> Self{
        let mut board = Board::default();
        
        // 初始化棋盘
        board.initialize();
        // 计算初始哈希值
        board.current_hash = board.zobrist_hash.calculate(&board);
        board.cache.cache_hash(board.current_hash);

        board
    }

    // 初始化棋盘
    fn initialize(&mut self){
        // 放置红方棋子
        self.set_piece_sync(Position::new(0, 0), Piece { piece_type: PieceType::Chariot, color: Color::Red });
        self.set_piece_sync(Position::new(1, 0), Piece { piece_type: PieceType::Horse, color: Color::Red });
        self.set_piece_sync(Position::new(2, 0), Piece { piece_type: PieceType::Elephant, color: Color::Red });
        self.set_piece_sync(Position::new(3, 0), Piece { piece_type: PieceType::Advisor, color: Color::Red });
        self.set_piece_sync(Position::new(4, 0), Piece { piece_type: PieceType::King, color: Color::Red });
        self.set_piece_sync(Position::new(5, 0), Piece { piece_type: PieceType::Advisor, color: Color::Red });
        self.set_piece_sync(Position::new(6, 0), Piece { piece_type: PieceType::Elephant, color: Color::Red });
        self.set_piece_sync(Position::new(7, 0), Piece { piece_type: PieceType::Horse, color: Color::Red });
        self.set_piece_sync(Position::new(8, 0), Piece { piece_type: PieceType::Chariot, color: Color::Red });
        
        self.set_piece_sync(Position::new(1, 2), Piece { piece_type: PieceType::Cannon, color: Color::Red });
        self.set_piece_sync(Position::new(7, 2), Piece { piece_type: PieceType::Cannon, color: Color::Red });
        
        self.set_piece_sync(Position::new(0, 3), Piece { piece_type: PieceType::Pawn, color: Color::Red });
        self.set_piece_sync(Position::new(2, 3), Piece { piece_type: PieceType::Pawn, color: Color::Red });
        self.set_piece_sync(Position::new(4, 3), Piece { piece_type: PieceType::Pawn, color: Color::Red });
        self.set_piece_sync(Position::new(6, 3), Piece { piece_type: PieceType::Pawn, color: Color::Red });
        self.set_piece_sync(Position::new(8, 3), Piece { piece_type: PieceType::Pawn, color: Color::Red });
        
        // 放置黑方棋子
        self.set_piece_sync(Position::new(0, 9), Piece { piece_type: PieceType::Chariot, color: Color::Black });
        self.set_piece_sync(Position::new(1, 9), Piece { piece_type: PieceType::Horse, color: Color::Black });
        self.set_piece_sync(Position::new(2, 9), Piece { piece_type: PieceType::Elephant, color: Color::Black });
        self.set_piece_sync(Position::new(3, 9), Piece { piece_type: PieceType::Advisor, color: Color::Black });
        self.set_piece_sync(Position::new(4, 9), Piece { piece_type: PieceType::King, color: Color::Black });
        self.set_piece_sync(Position::new(5, 9), Piece { piece_type: PieceType::Advisor, color: Color::Black });
        self.set_piece_sync(Position::new(6, 9), Piece { piece_type: PieceType::Elephant, color: Color::Black });
        self.set_piece_sync(Position::new(7, 9), Piece { piece_type: PieceType::Horse, color: Color::Black });
        self.set_piece_sync(Position::new(8, 9), Piece { piece_type: PieceType::Chariot, color: Color::Black });
        
        self.set_piece_sync(Position::new(1, 7), Piece { piece_type: PieceType::Cannon, color: Color::Black });
        self.set_piece_sync(Position::new(7, 7), Piece { piece_type: PieceType::Cannon, color: Color::Black });
        
        self.set_piece_sync(Position::new(0, 6), Piece { piece_type: PieceType::Pawn, color: Color::Black });
        self.set_piece_sync(Position::new(2, 6), Piece { piece_type: PieceType::Pawn, color: Color::Black });
        self.set_piece_sync(Position::new(4, 6), Piece { piece_type: PieceType::Pawn, color: Color::Black });
        self.set_piece_sync(Position::new(6, 6), Piece { piece_type: PieceType::Pawn, color: Color::Black });
        self.set_piece_sync(Position::new(8, 6), Piece { piece_type: PieceType::Pawn, color: Color::Black });
        
        // 更新王的位置
        self.update_king_positions();
    }


    // 更新王的缓存
    // 更新王的位置缓存
    fn update_king_positions(&mut self) {
        self.red_king_pos = self.find_king(Color::Red);
        self.black_king_pos = self.find_king(Color::Black);
    }
    
    // 查找王的位置
    pub fn find_king(&self, color: Color) -> Option<Position> {
        for x in 0..9 {
            for y in 0..10 {
                let pos = Position::new(x, y);
                let piece = self.get_piece(pos);
                if piece.piece_type == PieceType::King && piece.color == color {
                    return Some(pos);
                }
            }
        }
        None
    }

    pub fn get_piece(&self, pos: Position) -> Piece{
        if pos.is_valid(){
            self.pieces[pos.x() as usize][pos.y() as usize]
        }else{
            Piece::default()
        }
    }

    fn set_piece_sync(&mut self, pos: Position, piece: Piece){
        if pos.is_valid(){
            self.pieces[pos.x() as usize][pos.y() as usize] = piece;
            self.bitboard.set_piece(pos, piece);
        }
    }

    // 直接设置棋子（用于悔棋等操作）
    pub fn set_piece(&mut self, pos: Position, piece: super::piece::Piece) {
        if pos.is_valid() {
            self.pieces[pos.x() as usize][pos.y() as usize] = piece;
            self.bitboard.set_piece(pos, piece);
            
            // 如果设置的是王，更新缓存
            if piece.piece_type == super::piece::PieceType::King {
                self.update_king_positions();
            }
            
            // 更新哈希值
            self.current_hash = self.zobrist_hash.calculate(&self);
            self.cache.clear();
            self.cache.cache_hash(self.current_hash);
        }
    }

    pub fn make_move(&mut self, from: Position, to: Position){
        let moved_piece = self.get_piece(from);
        let captured_piece = self.get_piece(to);

        self.set_piece_sync(from, Piece::default());
        self.set_piece_sync(to, moved_piece);

        // 如果移动的是王或者吃掉的是王，更新缓存
        if moved_piece.piece_type == PieceType::King || captured_piece.piece_type == PieceType::King{
            self.update_king_positions();
        }

        self.current_hash = self.zobrist_hash.update(
            self.current_hash,
            from,
            to,
            moved_piece,
            captured_piece
        );

        self.cache.clear();
        self.cache.cache_hash(self.current_hash);
    }

    // 获取当前哈希值
    pub fn get_hash(&self) -> u64{
        self.current_hash
    }

    // 检查移动是否合法
    pub fn is_move_valid(&self, from: Position, to: Position, color: Color) -> bool {
        // 检查位置是否有效
        if !from.is_valid() || !to.is_valid(){
            return false;
        }

        // 检查起始位置是否存在棋子
        let from_piece = self.get_piece(from);
        if from_piece.piece_type == PieceType::None{
            return false;
        }

        // 检查棋子颜色是否正确
        if from_piece.color != color{
            return false;
        }

        // 检查目标位置是否有己方棋子
        let to_piece = self.get_piece(to);
        if to_piece.color == color{
            // 目标位置棋子所属不能是己方
            return false;
        }

        match from_piece.piece_type {
            PieceType::King => self.is_king_move_valid(from, to),
            PieceType::Advisor => self.is_advisor_move_valid(from, to),
            PieceType::Elephant => self.is_elephant_move_valid(from, to),
            PieceType::Horse => self.is_horse_move_valid(from, to),
            PieceType::Chariot => self.is_chariot_move_valid(from, to),
            PieceType::Cannon => self.is_cannon_move_valid(from, to),
            PieceType::Pawn => self.is_pawn_move_valid(from, to, color),
            _ => false,
        }
    }

    // 检查 Advisor 移动是否合法
    fn is_advisor_move_valid(&self, from: Position, to: Position) -> bool{
        let dx = (from.x() as i8 - to.x() as i8).abs();
        let dy = (from.y() as i8 - to.y() as i8).abs();

        // Advisor 只能九宫格内斜着走一步
        if !(dx == 1 && dy == 1){
            return false;
        }

        let piece = self.get_piece(from);
        self.bitboard.is_in_palace(to, piece.color)
    }

    // 检查王的移动是否合法
    fn is_king_move_valid(&self, from: Position, to: Position) -> bool{
        let dx = (from.x() as i8 - to.x() as i8).abs();
        let dy = (from.y() as i8 - to.y() as i8).abs();

        // 王只能走一步
        if !(dx == 1 && dy == 0 || dx == 0 && dy ==1){
            return false;
        }

        // 王只能在九宫格内移动
        let piece = self.get_piece(from);
        self.bitboard.is_in_palace(to, piece.color)
    }

    fn is_elephant_move_valid(&self, from: Position, to: Position) -> bool{
        let dx = (from.x() as i8 - to.x() as i8).abs();
        let dy = (from.y() as i8 - to.y() as i8).abs();

        // 象只能写着走两格子（田字的对角线）
        if !(dx == 2 && dy == 2){
            return false;
        }

        // 检查象眼是否被堵住
        let eye_x = (from.x() + to.x()) / 2;
        let eye_y = (from.y() + to.y()) / 2;
        let eye_pos = Position::new(eye_x, eye_y);
        if self.get_piece(eye_pos).piece_type != PieceType::None{
            return false;
        }

        // 象不能过河
        let piece = self.get_piece(from);
        match piece.color{
            Color::Red => to.y() < 5, 
            Color::Black => to.y() >= 5,
            _ => false,
        }
    }


    fn is_horse_move_valid(&self, from: Position, to: Position) -> bool{
        let dx = (from.x() as i8 - to.x() as i8).abs();
        let dy = (from.y() as i8 - to.y() as i8).abs();

        // 马走日字
        if !(dx == 1 && dy == 2 || dx == 2 && dy == 1){
            return false;
        }

        // 检查是否绊马腿
        let leg_x: u8;
        let leg_y: u8;

        if dx == 2{
            // 竖着的日
            leg_x = (from.x() + to.x()) / 2;
            leg_y = from.y(); 
        }else{
            // 横着的日
            leg_x = from.x();
            leg_y = (from.y() + to.y()) / 2;
        }

        let leg_pos = Position::new(leg_x, leg_y);
        self.get_piece(leg_pos).piece_type == PieceType::None 
    }

    // 检查车的移动是否符合规则
    fn is_chariot_move_valid(&self, from: Position, to: Position) -> bool{
        // 车只能直线移动
        if from.x() != to.x() && from.y() != to.y(){
            return false;
        }

        // 检查路径是否有阻挡
        self.is_path_clear(from, to)
    }

    // 检查炮的移动是否符合规则
    fn is_cannon_move_valid(&self, from: Position, to: Position) -> bool{
        // 炮只能直线移动
        if from.x() != to.x() && from.y() != to.y(){
            return false;
        }

        // 检查是否有棋子被吃掉
        let to_piece = self.get_piece(to);
        let has_capture = to_piece.piece_type != PieceType::None;

        // 计算路径上的棋子数量
        let piece_count = self.count_pieces_on_path(from, to);

        // 炮移动时，如果没有吃子，路径必须清空
        // 如果吃子，路径上必须有且只有一个棋子
        if has_capture{
            piece_count == 1
        }else{
            piece_count == 0
        }
    }

    // 检查兵的移动是否合法
    fn is_pawn_move_valid(&self, from: Position, to: Position, color: Color) -> bool{
        let dx = (from.x() as i8 - to.x() as i8).abs();
        let dy = (from.y() as i8 - to.y() as i8).abs();

        // 兵只能移动一步并且过河之前不能横着走
        if !(dx == 0 && dy == 1 || (dx == 1 && dy == 0 && self.is_pawn_crossed_river(from, color))){
            return false;
        }

        match color{
            Color::Red => to.y() > from.y(),
            Color::Black => to.y() < from.y(),
            _ => false,
        }
    }

    // 检查兵是否过河
    fn is_pawn_crossed_river(&self, pos: Position, color: Color) -> bool{
        match color{
            Color::Red => pos.y() >= 5,
            Color::Black => pos.y() < 5,
            _ => false,
        }
    }

    // 检查路径是否清空
    fn is_path_clear(&self, from: Position, to: Position) -> bool{
        self.count_pieces_on_path(from, to) == 0
    }

    // 计算路径上棋子的数量
    fn count_pieces_on_path(&self, from: Position, to: Position) -> u32{
        let mut count = 0;

        if from.x() == to.x(){
            // 垂直移动
            let start = std::cmp::min(from.y(), to.y()) + 1;
            let end = std::cmp::max(from.y(), to.y());

            for y in start..end{
                let pos = Position::new(from.x(), y);
                if self.get_piece(pos).piece_type != PieceType::None{
                    count += 1;
                }
            }
        }else if from.y() == to.y(){
            // 水平移动
            let start = std::cmp::min(from.x(), to.x()) + 1;
            let end = std::cmp::max(from.x(), to.x());

            for x in start..end{
                let pos = Position::new(x, from.y());
                if self.get_piece(pos).piece_type != PieceType::None{
                    count += 1;
                }
            }
        }
        count 
    }

    // 检查是否将军
    pub fn is_in_check(&self, color: Color) -> bool{
        // 首先检查缓存
        if let Some(status) = self.cache.get_cached_check_status(color){
            return status;
        }

        // 找到己方王的位置(使用缓存位置，避免重复搜索)
        let king_pos = match color{
            Color::Red => self.red_king_pos,
            Color::Black => self.black_king_pos,
            _ => return false,
        };

        let mut is_check = false;

        if let Some(king_pos) = king_pos {
            // 检查所有敌方棋子是否可以攻击到王
            let opponent_color = match color{
                Color::Red => Color::Black,
                Color::Black => Color::Red,
                _ => return false,
            };

            for x in 0..9{
                for y in 0..10{
                    let from = Position::new(x, y);
                    if self.bitboard.has_color(from, opponent_color){
                        let piece = self.get_piece(from);
                        if piece.piece_type != PieceType::None{
                            // 检查对方棋子是否可以移动到王的位置
                            if self.is_move_valid(from, king_pos, opponent_color){
                                is_check = true;
                                break;
                            }
                        }
                    }
                }
            
                if is_check{
                    break;
                }
            }
            
        }

        is_check
    }


    // 检查移动后是否仍然被将军
    pub fn is_move_safe(&mut self, from: Position, to: Position, color: Color) -> bool{
        if !self.is_move_valid(from, to, color){
            return false;
        }

        // 保存原始状态
        let original_from = self.get_piece(from);
        let original_to = self.get_piece(to);
        let original_hash = self.current_hash;
        let original_red_king = self.red_king_pos;
        let original_black_king = self.black_king_pos;

        // 执行移动
        self.make_move(from, to);

        let is_safe = !self.is_in_check(color);

        // 恢复原始状态
        self.set_piece_sync(from, original_from);
        self.set_piece_sync(to, original_to);
        self.current_hash = original_hash;
        self.red_king_pos = original_red_king;
        self.black_king_pos = original_black_king;

        // 恢复位棋盘
        self.bitboard = Bitboard::new();
        self.bitboard.init_special_positions();
        for x in 0..9{
            for y in 0..10{
                let pos = Position::new(x, y);
                let piece = self.get_piece(pos);
                if piece.piece_type != PieceType::None{
                    self.bitboard.set_piece(pos, piece);
                }
            }
        }

        self.cache.clear();
        self.cache.cache_hash(original_hash);

        is_safe
    }

    // 检查是否有合法移动
    pub fn has_legal_moves(&self, color: Color) -> bool{
        let mut temp_board = self.clone();

        for x1 in 0..9{
            for y1 in 0..10{
                let from = Position::new(x1, y1);
                let from_piece = self.get_piece(from);

                if from_piece.color == color{
                    for x2 in 0..9{
                        for y2 in 0..10{
                            let to = Position::new(x2, y2);
                            if self.is_move_valid(from, to, color){
                                if temp_board.is_move_safe(from, to, color){
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }


    // 检查是否将死
    pub fn is_checkmate(&self, color: Color) -> bool {
        // 首先检查是否被将军
        if !self.is_in_check(color) {
            return false;
        }
        
        // 然后检查是否没有合法移动
        !self.has_legal_moves(color)
    }
    
    // 检查是否困毙
    pub fn is_stalemate(&self, color: Color) -> bool {
        // 首先检查是否没有被将军
        if self.is_in_check(color) {
            return false;
        }
        
        // 然后检查是否没有合法移动
        !self.has_legal_moves(color)
    }
}