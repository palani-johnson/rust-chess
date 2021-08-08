use std::fs::File;
use std::io::{Result, Write};

fn main() {
    gen_cache_file();
    println!("cargo:rerun-if-changed=build.rs");
}

fn gen_cache_file() -> Result<()> {
    let mut f = File::create("src/chess/cache.rs")?;

    write!(f, "
pub const PLAYERS_START: [u64; 2] = [
    0x000000000000ffff, // Player 0
    0xffff000000000000  // Player 1
];

pub const PIECES_START: [u64; 6] = [
    0x1000000000000010, // Kings
    0x0800000000000008, // Queens
    0x2400000000000024, // Bishops
    0x8100000000000081, // Rooks
    0x4200000000000042, // Knights
    0x00ff00000000ff00, // Pawns            
];

pub const KNIGHT_CACHE: [u64; 64] = {:#0x?};

pub const KING_CACHE: [u64; 64] = {:#0x?};

pub const RANKS: [u64; 8] = {:#0x?};

pub const PAWN_ATTACKS_P0: [u64; 64] = {:#0x?};

pub const PAWN_ATTACKS_P1: [u64; 64] = {:#0x?};",
    gen_knight_cache(),
    gen_king_cache(),
    gen_ranks(),
    gen_pawn_attacks_p0(),
    gen_pawn_attacks_p1())?;

    Ok(())
}

fn gen_pawn_attacks_p0() -> [u64; 64] {
    moves_from_pairs(vec![
        (1, 1), (1, -1), 
    ])
}

fn gen_pawn_attacks_p1() -> [u64; 64] {
    moves_from_pairs(vec![
        (-1, 1), (-1, -1), 
    ])
}

fn gen_ranks() -> [u64; 8] {
    let mut i = 0;
    return [0xff; 8].map(|r| {
        let rank = r<<(8*i);
        i += 1;
        rank
    });
}

fn gen_knight_cache() -> [u64; 64] {
    moves_from_pairs(vec![
        (1, 2), (2, 1), 
        (1, -2), (2, -1),
        (-1, 2), (-2, 1), 
        (-1, -2), (-2, -1)
    ])
}

fn gen_king_cache() -> [u64; 64] {
    moves_from_pairs(vec![
        (1, 1), (-1, -1), 
        (0, 1), (0, -1),
        (1, 0), (-1, 0), 
        (-1, 1), (1, -1)
    ])
}

fn moves_from_pairs(pairs: Vec<(i32, i32)>) -> [u64; 64] {
    let mut moves = [0; 64];

    for r in 0..=7 {
        for c in 0..=7 {
            let n = 8*r + c;
            for pair in &pairs {
                moves[n] |= match (pair.0 + r as i32, pair.1 + c as i32) {
                    (0..=7, 0..=7) => 1 << 8*(pair.0 + r as i32) + (pair.1 + c as i32),
                    _ => 0
                }
            }
        }
    }

    return moves;
}