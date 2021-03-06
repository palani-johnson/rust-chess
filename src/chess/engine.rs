use crate::chess::cache::*;


pub struct Board {
    // Player bitboards: [player1, player2]
    pub players: [u64; 2],
    // Pieces bitboards: [kings, queens, bishops, rooks, knights, pawns]
    pub pieces: [u64; 6],

    // board states
    pub turn: bool,
    pub checkmate: bool,
    pub enpassant: u64,
    pub enpassant_pos: (usize, usize),

    // moves
    pub moves: [u64; 64],
}

impl Board {
    pub fn init() -> Board { 
        let mut board = Board {
            players: PLAYERS,
            pieces: PIECES,
            checkmate: false,
            turn: true,
            enpassant: 0,
            enpassant_pos: (64, 64),
            moves: [0; 64],
        };
        board.gen_moves();
        return board;
    }

    pub fn perform_move(&self, p0: usize, p1: usize) -> Board {
        let player_num = self.players.iter().enumerate().find(|p| {
            1<<p0 & p.1 != 0
        });
        let piece_num = self.pieces.iter().enumerate().find(|p| {
            1<<p0 & p.1 != 0
        });

        let remove_pieces: Box<dyn Fn(u64) -> u64>  = if 
            (self.enpassant_pos.1 == p1)
            && (((p0+1 == self.enpassant_pos.0) 
            && (self.pieces[5] & 1<<(p0+1) != 0)) 
            || ((p0-1 == self.enpassant_pos.0) 
            && (self.pieces[5] & 1<<(p0-1) != 0))) 
        {
            Box::new(|p| p & !(1<<p0 | 1<<p1 | 1<<self.enpassant_pos.0))
        } else {
            Box::new(|p| p & !(1<<p0 | 1<<p1))
        };
        
          
        let mut players = self.players.map(&remove_pieces);
        let mut pieces = self.pieces.map(remove_pieces);

        let mut enpassant = 0;
        let mut enpassant_pos = (64, 64);
        if let Some((i, _)) = player_num {
            players[i] |= 1<<p1;
            if let Some((j, _)) = piece_num {
                pieces[j] |= 1<<p1;
                if j == 5 && (p0 as isize - p1 as isize).abs() == 16 {
                    enpassant_pos = (p1, match self.turn {
                        true => p1-8,
                        false => p1+8,
                    });
                    enpassant |= 1<<enpassant_pos.1;
                } 
            }
        }

        let mut board = Board {
            players: players,
            pieces: pieces,
            checkmate: false,
            turn: !self.turn,
            enpassant: enpassant,
            enpassant_pos: enpassant_pos,
            moves: [0; 64]
        };
        board.gen_moves();
        return board;
    }

    fn gen_moves(&mut self) {
        type PawnFn = fn(usize, usize) -> u64;
        let (us, them, pawn_attacks, pawn_home, pawn_fn) = match self.turn {
            true => (
                self.players[0], 
                self.players[1], 
                *PAWN_ATTACKS_P0,
                8..16,
                ( |p, n| 1u64 << (p+n) ) as PawnFn,
            ),
            false => (
                self.players[1], 
                self.players[0], 
                *PAWN_ATTACKS_P1,
                48..56,
                ( |p, n| 1u64 << (p-n) ) as PawnFn,
            ),
        };

        let occupied = us | them;

        let our_pieces = self.pieces.map(|p| p & us);

        for (i, &(mut piece)) in our_pieces.iter().enumerate() {    
            while let pos@0..=63 = piece.trailing_zeros() as usize {
                self.moves[pos] = match i {
                    0 => !us & KING_ATTACKS[pos],
                    1 => !us & (bishop_moves(pos, occupied) | rook_moves(pos, occupied)),
                    2 => !us & bishop_moves(pos, occupied),
                    3 => !us & rook_moves(pos, occupied),
                    4 => !us & KNIGHT_ATTACKS[pos],
                    5 => {
                        let mut mv = !occupied & pawn_fn(pos, 8);
                        if mv != 0 && pawn_home.contains(&pos) {
                            mv |= !occupied & pawn_fn(pos, 16);
                        }
                        mv | ((them | self.enpassant) & pawn_attacks[pos])
                    },
                    _ => 0
                };
                piece ^= 1 << pos;
            }
        }
    }
}

fn rook_moves(pos: usize, mut occupied: u64) -> u64 {
    occupied &= ROOK_MASKS[pos];
    occupied *= ROOK_MAGICS[pos];
    occupied >>= ROOK_SHIFTS[pos];
    return ROOK_ATTACKS[pos][occupied as usize];
}

fn bishop_moves(pos: usize, mut occupied: u64) -> u64 {
    occupied &= BISHOP_MASKS[pos];
    occupied *= BISHOP_MAGICS[pos];
    occupied >>= BISHOP_SHIFTS[pos];
    return BISHOP_ATTACKS[pos][occupied as usize];
}
