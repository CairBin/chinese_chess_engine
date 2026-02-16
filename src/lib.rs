use libc;

pub mod core;
pub mod parser;

pub use core::ruler::{Ruler, GameStatus};
pub use core::board::Board;
pub use core::position::Position;
pub use core::piece::{Color, Piece, PieceType};
pub use parser::parser::Parser;
pub use parser::ast::ASTNode;

#[derive(Debug, Clone)]
pub enum EngineResult {
    // 成功
    GameCreated(u32),
    MoveSuccess(u32),
    JoinSuccess(u32),
    UndoSuccess(u32),
    GameStatus(u32, GameStatus),
    
    // 错误
    GameNotFound(u32),
    MoveFailed(u32),
    JoinFailed(u32),
    UndoFailed(u32),
    InvalidCommand,
}

impl std::fmt::Display for EngineResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineResult::GameCreated(game_id) => write!(f, "GAME_CREATED {}", game_id),
            EngineResult::MoveSuccess(game_id) => write!(f, "MOVE_SUCCESS {}", game_id),
            EngineResult::JoinSuccess(game_id) => write!(f, "JOIN_SUCCESS {}", game_id),
            EngineResult::UndoSuccess(game_id) => write!(f, "UNDO_SUCCESS {}", game_id),
            EngineResult::GameStatus(game_id, status) => write!(f, "GAME_STATUS {} {:?}", game_id, status),
            EngineResult::GameNotFound(game_id) => write!(f, "GAME_NOT_FOUND {}", game_id),
            EngineResult::MoveFailed(game_id) => write!(f, "MOVE_FAILED {}", game_id),
            EngineResult::JoinFailed(game_id) => write!(f, "JOIN_FAILED {}", game_id),
            EngineResult::UndoFailed(game_id) => write!(f, "UNDO_FAILED {}", game_id),
            EngineResult::InvalidCommand => write!(f, "INVALID_COMMAND"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Engine {
    game_manager: Ruler,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            game_manager: Ruler::new(),
        }
    }
    
    /// 解析命令位AST节点
    pub fn parse(&self, command: &str) -> ASTNode {
        let mut parser = Parser::new(command);
        parser.parse()
    }
    
    /// 执行语句
    pub fn execute(&mut self, command: &str) -> EngineResult {
        let ast_node = self.parse(command);
        self.execute_ast(ast_node)
    }
    
    /// 执行AST节点指令并返回结果
    pub fn execute_ast(&mut self, ast_node: ASTNode) -> EngineResult {
        match ast_node {
            ASTNode::CreateGame => {
                let game_id = self.game_manager.create_game();
                EngineResult::GameCreated(game_id)
            }
            ASTNode::JoinGame { game_id, name, color } => {
                if self.game_manager.add_player_to_game(game_id, name, color) {
                    EngineResult::JoinSuccess(game_id)
                } else {
                    EngineResult::JoinFailed(game_id)
                }
            }
            ASTNode::Move { game_id, color: _, from_x, from_y, to_x, to_y } => {
                if self.game_manager.make_move(game_id, from_x, from_y, to_x, to_y) {
                    EngineResult::MoveSuccess(game_id)
                } else {
                    EngineResult::MoveFailed(game_id)
                }
            }
            ASTNode::Undo { game_id } => {
                if self.game_manager.undo_move(game_id) {
                    EngineResult::UndoSuccess(game_id)
                } else {
                    EngineResult::UndoFailed(game_id)
                }
            }
            ASTNode::GetGame { game_id } => {
                if let Some(game) = self.game_manager.get_game(game_id) {
                    EngineResult::GameStatus(game_id, game.status.clone())
                } else {
                    EngineResult::GameNotFound(game_id)
                }
            }
            ASTNode::Invalid => {
                EngineResult::InvalidCommand
            }
        }
    }
    
    /// 获取游戏管理者
    pub fn get_game_manager(&self) -> &Ruler {
        &self.game_manager
    }
    
    /// 获取游戏管理者可变引用
    pub fn get_game_manager_mut(&mut self) -> &mut Ruler {
        &mut self.game_manager
    }
}

/// 创造引擎实例
pub fn new() -> Engine {
    Engine::new()
}

/// 创建管理者
pub fn create_game_manager() -> Ruler {
    Ruler::new()
}

/// 创建棋盘
pub fn create_board() -> Board {
    Board::new()
}

/// 解析命令
pub fn parse_command(command: &str) -> ASTNode {
    let mut parser = Parser::new(command);
    parser.parse()
}

// C API

#[repr(C)]
pub struct CECEngine {
    _private: [u8; 0],
}

/// 返回结果
#[repr(C)]
pub struct CECEngineResult {
    _private: [u8; 0],
}

/// 创建引擎对象
#[unsafe(no_mangle)]
pub extern "C" fn cec_engine_new() -> *mut CECEngine {
    let engine = Box::new(Engine::new());
    Box::into_raw(engine) as *mut CECEngine
}

/// 销毁引擎对象
#[unsafe(no_mangle)]
pub extern "C" fn cec_engine_free(engine: *mut CECEngine) {
    if !engine.is_null() {
        unsafe {
            let _ = Box::from_raw(engine as *mut Engine);
        }
    }
}

/// 执行命令
#[unsafe(no_mangle)]
pub extern "C" fn cec_engine_execute(engine: *mut CECEngine, command: *const libc::c_char) -> *mut CECEngineResult {
    if engine.is_null() || command.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let engine = &mut *(engine as *mut Engine);
        let command = std::ffi::CStr::from_ptr(command).to_str().unwrap_or("");
        let result = engine.execute(command);
        let result_box = Box::new(result);
        Box::into_raw(result_box) as *mut CECEngineResult
    }
}

/// 获取结果并转换成字符串
#[unsafe(no_mangle)]
pub extern "C" fn cec_result_to_string(result: *mut CECEngineResult, buffer: *mut libc::c_char, buffer_size: libc::size_t) {
    if result.is_null() || buffer.is_null() || buffer_size == 0 {
        return;
    }
    
    unsafe {
        let result = &*(result as *mut EngineResult);
        let result_str = format!("{}", result);
        let result_c_str = std::ffi::CString::new(result_str).unwrap_or(std::ffi::CString::new("").unwrap());
        libc::strncpy(buffer, result_c_str.as_ptr(), buffer_size - 1);
        *buffer.offset((buffer_size - 1) as isize) = 0;
    }
}

/// 释放引擎对象
#[unsafe(no_mangle)]
pub extern "C" fn cec_result_free(result: *mut CECEngineResult) {
    if !result.is_null() {
        unsafe {
            let _ = Box::from_raw(result as *mut EngineResult);
        }
    }
}
