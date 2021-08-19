use crate::chess::cache::*;

pub struct Board {
    // Player bitboards: [player1, player2]
    pub players: [u64; 2],
    // Pieces bitboards: [kings, queens, bishops, rooks, knights, pawns]
    pub pieces: [u64; 6],

    // board states
    pub turn: bool,
    pub checkmate: bool,
    enpassant: u64,

    // moves
    pub moves: [u64; 64],
}

impl Board {
    pub fn init() -> Board { 
        let mut board = Board {
            players: PLAYERS_START,
            pieces: PIECES_START,
            checkmate: false,
            turn: true,
            enpassant: 0,
            moves: [0; 64]
        };
        board.gen_moves();
        return board;
    }

    pub fn perform_move(&self, p0: usize, p1: usize) -> Board {
        let player_num = self.players.iter().enumerate().find(|p| 1<<p0 & p.1 != 0);
        let piece_num = self.pieces.iter().enumerate().find(|p| 1<<p0 & p.1 != 0);
          
        let mut players = self.players.map(|p| p & !(1<<p0 | 1<<p1));
        let mut pieces = self.pieces.map(|p| p & !(1<<p0 | 1<<p1));

        if let Some((i, _)) = player_num {players[i] |= 1<<p1;}

        let mut enpassant = 0;
        if let Some((i, _)) = piece_num {
            if i == 5 && (p1 as isize - p0 as isize).abs() > 10 {
                enpassant |= 1<<p1;
            }
            pieces[i] |= 1<<p1;
        }

        let mut board = Board {
            players: players,
            pieces: pieces,
            checkmate: false,
            turn: !self.turn,
            enpassant: enpassant,
            moves: [0; 64]
        };
        board.gen_moves();
        return board;
    }

    fn gen_moves(&mut self) {
        let (us, them, pawn_attacks, pawn_home) = match self.turn {
            true => (
                self.players[0], 
                self.players[1], 
                PAWN_ATTACKS_P0,
                8..16
            ),
            false => (
                self.players[1], 
                self.players[0], 
                PAWN_ATTACKS_P1,
                48..56
            ),
        };

        let pawn_fn: Box<dyn Fn(usize, usize) -> u64> = match self.turn {
            true => Box::new(|p, n| 1<<(p+n)),
            false => Box::new(|p, n| 1<<(p-n)),
        };

        let any = us | them;

        let our_pieces = self.pieces.map(|p| p & us);

        for (i, &(mut piece)) in our_pieces.iter().enumerate() {    
            while let pos@0..=63 = piece.trailing_zeros() as usize {
                self.moves[pos] = match i {
                    0 => !us & KING_CACHE[pos],
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => !us & KNIGHT_CACHE[pos],
                    5 => {
                        let mut mv = !any & pawn_fn(pos, 8);
                        if mv != 0 && pawn_home.contains(&pos) {
                            mv |= !any & pawn_fn(pos, 16);
                        }
                        mv | (pawn_attacks[pos] & them)
                    },
                    _ => 0
                };
                piece ^= 1 << pos;
            }
        }
    }
}