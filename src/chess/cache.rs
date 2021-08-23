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
        (-1, -2), (-2, -1),
    ]);

    pub static ref KING_CACHE: [u64; 64] = cache_moves(vec![
        (1, 1), (-1, -1), 
        (0, 1), (0, -1),
        (1, 0), (-1, 0), 
        (-1, 1), (1, -1),
    ]);

    pub static ref PAWN_ATTACKS_P0: [u64; 64] = cache_moves(vec![
        (1, 1), (1, -1),
    ]);

    pub static ref PAWN_ATTACKS_P1: [u64; 64] = cache_moves(vec![
        (-1, 1), (-1, -1),
    ]);

    pub static ref ROOK_BLOCKER_RAYS: [u64; 64] = cache_rays(vec![
        (0, 1), (0, -1),
        (1, 0), (-1, 0),
    ]);

    pub static ref BISHOP_BLOCKER_RAYS: [u64; 64] = cache_rays(vec![
        (1, 1), (-1, -1), 
        (-1, 1), (1, -1),
    ]);

    pub static ref ROOK_MAGICS: [u64; 64] = [0; 64];

    pub static ref ROOK_MAGICS: [u64; 64] = [0; 64];
}


fn cache_moves(pairs: Vec<(i32, i32)>) -> [u64; 64] {
    let mut moves = [0; 64];

    for n in 0..64 {
        for (rank_step, file_step) in &pairs {
            moves[n as usize] |= stepper(
                n / 8,
                n % 8,
                *rank_step,
                *file_step,
                false
            );
        }
    }

    return moves;
}

fn cache_rays(pairs: Vec<(i32, i32)>) -> [u64; 64] {
    let mut moves = [0; 64];

    for n in 0..64 {
        let mut m = 0;
        let rank = n / 8;
        let file = n % 8;

        for (rank_step, file_step) in &pairs {
            m |= stepper(
                rank,
                file,
                *rank_step,
                *file_step,
                true,
            );
        }

        if rank != 0 { m &= !0x00000000000000ff }
        if rank != 7 { m &= !0xff00000000000000 }
        if file != 0 { m &= !0x0101010101010101 }
        if file != 7 { m &= !0x8080808080808080 }

        moves[n as usize] = m;
    }

    return moves;
}

fn stepper(mut rank: i32, mut file: i32, rank_step:  i32, file_step: i32, ray: bool) -> u64 {
    let mut m = 0;

    loop {
        rank += rank_step;
        file += file_step;
        match (rank, file) {
            (0..=7, 0..=7) => m |= 1 << 8*rank + file,
            _ => break
        };

        if !ray { break }
    }

    return m;
}