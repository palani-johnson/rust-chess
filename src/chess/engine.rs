pub struct Board {
    // Player bitboards: [player1, player2]
    pub players: [u64; 2],

    // Pieces bitboards: [pawns, knights, rooks, bishops, queens, kings]
    pub pieces: [u64; 6],

    pub checkmate: bool,

    pub moves: [u64; 64],

    pub turn: bool,

}

impl Board {
    pub fn init() -> Board {
        let players = [
            0x000000000000ffff, // Player 0
            0xffff000000000000  // Player 1
        ];

        let pieces = [
            0x1000000000000010, // Kings
            0x0800000000000008, // Queens
            0x2400000000000024, // Bishops
            0x8100000000000081, // Rooks
            0x4200000000000042, // Knights
            0x00ff00000000ff00, // Pawns            
        ];

        Board {
            players: players,
            pieces: pieces,
            checkmate: false,
            moves: gen_moves(pieces.map(|p| p & players[0]), true),
            turn: true
        }
    }

    pub fn perform_move(&self, p0: usize, p1: usize) -> Board {
        let player_num = self.players.iter().enumerate().find(|p| 1<<p0 & p.1 != 0);
        let piece_num = self.pieces.iter().enumerate().find(|p| 1<<p0 & p.1 != 0);
          
        let mut players = self.players.map(|p| p & !(1<<p0 | 1<<p1));
        let mut pieces = self.pieces.map(|p| p & !(1<<p0 | 1<<p1));

        if let Some((i, _)) = player_num { players[i] |= 1<<p1; }
        if let Some((i, _)) = piece_num { pieces[i] |= 1<<p1; }

        let turn = !self.turn;

        Board {
            players: players,
            pieces: pieces,
            checkmate: false,
            turn: turn,
            moves: gen_moves(
                pieces.map(|p| 
                    p & if turn {players[0]} else {players[1]}
                ),
                turn
            )
        }
    }
}

fn gen_moves(mapped_pieces: [u64; 6], turn: bool) -> [u64; 64] {
    let mut moves = [0; 64];

    for (i, piece) in mapped_pieces.iter().enumerate() {
        let mut piece = *piece;
        while let pos@0..=63 = piece.trailing_zeros() as usize {
            moves[pos] = match i {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 4,
                5 => 5,
                _ => 0
            };
            piece ^= 1 << pos;
        }
    }

    return moves;
}