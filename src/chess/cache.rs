use crate::chess::engine::Board;

pub const START_BOARD: Board = Board {
    players: [
        0x000000000000ffff, // Player 0
        0xffff000000000000  // Player 1
    ],
    pieces: [
        0x1000000000000010, // Kings
        0x0800000000000008, // Queens
        0x2400000000000024, // Bishops
        0x8100000000000081, // Rooks
        0x4200000000000042, // Knights
        0x00ff00000000ff00, // Pawns            
    ],
    turn: true,
    checkmate: false,
    moves: [0; 64]
};

pub const KNIGHT_CACHE: [u64; 64] = [0; 64];

pub const KING_CACHE: [u64; 64] = [0; 64];