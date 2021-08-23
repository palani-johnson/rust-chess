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
    0x00ff00000000ff00  // Pawns            
];

lazy_static! {
    pub static ref KNIGHT_CACHE: [u64; 64] = cache_moves(vec![
        (1, 2), (2, 1), 
        (1, -2), (2, -1),
        (-1, 2), (-2, 1), 
        (-1, -2), (-2, -1)
    ]);

    pub static ref KING_CACHE: [u64; 64] = cache_moves(vec![
        (1, 1), (-1, -1), 
        (0, 1), (0, -1),
        (1, 0), (-1, 0), 
        (-1, 1), (1, -1)
    ]);

    pub static ref PAWN_ATTACKS_P0: [u64; 64] = cache_moves(vec![
        (1, 1), (1, -1)
    ]);

    pub static ref PAWN_ATTACKS_P1: [u64; 64] = cache_moves(vec![
        (-1, 1), (-1, -1)
    ]);

    pub static ref ROOK_MAGIC_RAYS: [u64; 64] = cache_magic_rays(vec![
        (0, 1), (0, -1),
        (1, 0), (-1, 0)
    ]);

    pub static ref BISHOP_MAGIC_RAYS: [u64; 64] = cache_magic_rays(vec![
        (1, 1), (-1, -1), 
        (-1, 1), (1, -1)
    ]);
}


fn cache_moves(pairs: Vec<(i32, i32)>) -> [u64; 64] {
    let mut moves = [0; 64];

    for rank in 0..=7 {
        for file in 0..=7 {
            let n = 8*rank + file;
            for pair in &pairs {
                let (r, f) = (
                    pair.0 + rank as i32, 
                    pair.1 + file as i32
                );
                moves[n] |= match (r, f) {
                    (0..=7, 0..=7) => 1 << 8*r + f,
                    _ => 0
                };
            }
        }
    }

    return moves;
}

fn cache_magic_rays(pairs: Vec<(i32, i32)>) -> [u64; 64] {
    let mut moves = [0; 64];

    for mut rank in 0..=7  {
        for mut file in 0..=7 {
            let n = 8*rank + file;
            for pair in &pairs {
                let (r, f) = (
                    pair.0 + rank, 
                    pair.1 + file
                );
                match (r, f) {
                    (0..=7, 0..=7) => moves[n as usize] |= 1 << 8*r + f,
                    _ => break,
                }
                rank = r;
                file = f;
            }
        }
    }

    return moves;
}