mod engine;
mod term;
mod cache;

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

        let mut _p0 = 64; // using underscore to trick linter
        match get_pos() {
            Action::Esc => return,
            Action::Back => continue,
            Action::Pos(p@0..=63) => _p0 = p,
            _ => {
                history.push(board);
                continue;
            },
        }


        let move_bb = board.moves[_p0];
        print_bitboard(move_bb, 2, 6);
        if move_bb != 0 {
            for (piece, piece_bb) in board.pieces.iter().enumerate() {
                print_bitboard(move_bb & piece_bb, 2, piece);
            }
        } else {
            history.push(board);
            continue;
        }
        
        let mut _p1 = 64; // using underscore to trick linter
        match get_pos() {
            Action::Esc => return,
            Action::Back => continue,
            Action::Pos(p@0..=63) => _p1 = p,
            _ => {
                history.push(board);
                continue;
            },
        }

        if 1<<_p1 & move_bb != 0 {
            print_move(_p0, _p1);

            let next = board.perform_move(_p0, _p1);
            history.push(board);
            history.push(next);
        } else {
            history.push(board);
        }
    }

}