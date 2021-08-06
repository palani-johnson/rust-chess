mod engine;
mod term;

use term::*;
use engine::Board;

pub fn term_run() {
    init_term();

    let mut history = vec![Board::init()];

    loop {
        let board: Board = history.pop().unwrap();
        reset_board();

        for (player, player_bb) in board.players.iter().enumerate() {
            for (piece, piece_bb) in board.pieces.iter().enumerate() {
                print_bitboard(player_bb & piece_bb, player, piece);
            }
        }

        let mut p0 = 64;
        match get_pos() {
            ExitType::Esc => return,
            ExitType::Again => {
                history.push(board);
                continue;
            },
            ExitType::Pos(p@0..=63) => p0 = p,
            ExitType::Pos(_) => panic!(),
        }


        let move_bb = board.moves[p0];
        print_bitboard(move_bb, 2, 6);
        if move_bb != 0 {
            for (piece, piece_bb) in board.pieces.iter().enumerate() {
                print_bitboard(move_bb & piece_bb, 2, piece);
            }
        } else {
            history.push(board);
            continue;
        }
        
        let mut p1 = 64;
        match get_pos() {
            ExitType::Esc => return,
            ExitType::Again => {
                history.push(board);
                continue;
            },
            ExitType::Pos(p@0..=63) => p1 = p,
            ExitType::Pos(_) => panic!(),
        }

        if 1<<p1 & move_bb != 0 {
            print_move(p0, p1);

            let next = board.perform_move(p0, p1);
            history.push(board);
            history.push(next);
        } else {
            history.push(board);
        }
    }

}