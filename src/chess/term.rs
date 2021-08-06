extern crate termion;

use termion::clear;
use termion::color::{Bg, Fg, AnsiValue};
use termion::style::Reset;
use termion::cursor::{Goto, Left};
use termion::input::{MouseTerminal};
use termion::raw::IntoRawMode;
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead};

use std::io::{stdin};
use std::io::{Write, stdout};

use std::char;


const FG_COLORS: [Fg<AnsiValue>; 3] = [Fg(AnsiValue(92)), Fg(AnsiValue(30)), Fg(AnsiValue(160))];
const BG_COLORS: [Bg<AnsiValue>; 2] = [Bg(AnsiValue(207)), Bg(AnsiValue(226))];
const ICON_LIST: [char; 7] = ['♛', '♚', '♝', '♜', '♞', '♟', '·'];


pub fn init_term() {
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}Rust chess v0.1.0{}", 
        clear::All, 
        Goto(1, 1),
        Goto(2, 3)
    ).unwrap();

    for i in (1..9).rev() {
        write!(stdout, "{}\n{}", 
            i,
            Left(1)
        ).unwrap()
    }

    write!(stdout, " ABCDEFGH").unwrap();
    stdout.flush().unwrap();
}


pub fn reset_board() {
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    for pos in 0..64 {
        write!(stdout, "{}{} {}",
            Goto((3+pos%8) as u16, (10-pos/8) as u16),
            BG_COLORS[((pos/8 + pos%8) % 2) as usize],
            Reset
        );
    }

    stdout.flush().unwrap();
}


pub fn print_bitboard(bitboard: u64, player: usize, piece: usize) {
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let mut bitboard = bitboard;

    while bitboard != 0 {
        let pos = bitboard.trailing_zeros();

        write!(stdout, "{}{}{}{}{}",
            Goto((3+pos%8) as u16, (10-pos/8) as u16),
            BG_COLORS[((pos/8 + pos%8) % 2) as usize],
            FG_COLORS[player],
            ICON_LIST[piece],
            Reset
        ).unwrap();

        bitboard ^= 1 << pos;
    }

    stdout.flush().unwrap();
}


pub enum Action {
    Pos(usize),
    Again,
    Esc,
    Back
}

pub fn get_pos() -> Action {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}Esc to exit. Del to undo.", Goto(1, 15)).unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Esc) => return Action::Esc,
            Event::Key(Key::Delete) => return Action::Back,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x@3..=10, y@3..=10) => {
                        let rank = 10 - y;
                        let file = x - 3;
                        return Action::Pos((file + 8*rank) as usize);
                    },
                    MouseEvent::Press(_, _, _) => return Action::Again,
                    _ => (),
                }
            }
            _ => (),
        }
    }

    return Action::Again;
}

pub fn print_move(p0: usize, p1: usize) {
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let rf_format = |p| format!("{}{}", char::from_u32((65+p%8) as u32).unwrap(), p/8 +1);

    write!(stdout, "{}Moved {} to {}{}", 
        Goto(13, 3), 
        rf_format(p0),
        rf_format(p1),
        Goto(1, 14)
    ).unwrap();
    stdout.flush().unwrap();
}