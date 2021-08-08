use crate::chess::cache::*;

pub struct Board {
    // Player bitboards: [player1, player2]
    pub players: [u64; 2],

    // Pieces bitboards: [kings, queens, bishops, rooks, knights, pawns]
    pub pieces: [u64; 6],

    pub turn: bool,
    pub checkmate: bool,

    pub moves: [u64; 64],
}

impl Board {
    pub fn init() -> Board { 
        let mut board = Board {
            players: PLAYERS_START,
            pieces: PIECES_START,
            checkmate: false,
            turn: true,
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

        if let Some((i, _)) = player_num { players[i] |= 1<<p1; }
        if let Some((i, _)) = piece_num { pieces[i] |= 1<<p1; }

        let mut board = Board {
            players: players,
            pieces: pieces,
            checkmate: false,
            turn: !self.turn,
            moves: [0; 64]
        };
        board.gen_moves();
        return board;
    }

    fn gen_moves(&mut self) {
        let (us, them) = match self.turn {
            true => (self.players[0], self.players[1]),
            false => (self.players[1], self.players[0]),
        };

        let any = us | them;

        let our_pieces = self.pieces.map(|p| p&us);

        for (i, piece) in our_pieces.iter().enumerate() {
            let mut piece = *piece;
    
            while let pos@0..=63 = piece.trailing_zeros() as usize {
                self.moves[pos] = match i {
                    0 => !us & KING_CACHE[pos],
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => !us & KNIGHT_CACHE[pos],
                    5 => match self.turn {
                        true => {
                            let mut mv = !any & 1<<(pos+8);
                            if mv != 0 && (8..16).contains(&pos) {
                                mv |= !any & 1<<(pos+16);
                            }
                            mv | (PAWN_ATTACKS_P0[pos] & them)
                        },
                        false => {
                            let mut mv = !any & 1<<(pos-8);
                            if mv != 0 && (48..56).contains(&pos) {
                                mv |= !any & 1<<(pos-16);
                            }
                            mv | (PAWN_ATTACKS_P1[pos] & them)
                        }
                    },
                    _ => 0
                };
                piece ^= 1 << pos;
            }
        }
    }
}