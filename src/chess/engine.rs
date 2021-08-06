extern crate rand;

use rand::Rng;

pub struct Board {
    // Player bitboards: [player1, player2]
    pub players: [u64; 2],

    // Pieces bitboards: [pawns, knights, rooks, bishops, queens, kings]
    pub pieces: [u64; 6],

    pub checkmate: bool,

    pub moves: [u64; 64]

}

impl Board {
    pub fn init() -> Board {
        let players = [
            0x000000000000ffff, 
            0xffff000000000000
        ];

        let pieces = [
            0x00ff00000000ff00, 
            0x4200000000000042, 
            0x8100000000000081, 
            0x2400000000000024,
            0x0800000000000008, 
            0x1000000000000010
        ];

        let moves = Board::gen_moves();

        Board {
            players: players,
            pieces: pieces,
            checkmate: false,
            moves: moves,
        }
    }

    pub fn perform_move(&self, p0: usize, p1: usize) -> Board {
        let player_num = self.players.iter().enumerate().find(|p| 1 << p0 & p.1 != 0);
        let piece_num = self.pieces.iter().enumerate().find(|p| 1 << p0 & p.1 != 0);
          
        let mut players = self.players.map(|p| p & !(1<<p0 | 1<<p1));
        let mut pieces = self.pieces.map(|p| p & !(1<<p0 | 1<<p1));

        if let Some((i, _)) = player_num { players[i] |= 1 << p1; }
        if let Some((i, _)) = piece_num { pieces[i] |= 1 << p1; }

        let moves = Board::gen_moves();

        Board {
            players: players,
            pieces: pieces,
            checkmate: false,
            moves: moves,
        }
    }

    fn gen_moves() -> [u64; 64] {
        let mut moves: [u64; 64] = [0; 64];
        let mut rng = rand::thread_rng();
        for i in 0..64 {
            moves[i] = rng.gen();
        }

        return moves;
    }
}