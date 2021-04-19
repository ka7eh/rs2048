use rs2048::*;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut game = Game::new(4, 2);
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}q to quit\r\n{}",
        termion::cursor::Hide,
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        game
    )
    .unwrap();
    stdout.flush().unwrap();

    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char('q') => break,
            Key::Left => game.move_tiles(Direction::Left),
            Key::Right => game.move_tiles(Direction::Right),
            Key::Up => game.move_tiles(Direction::Up),
            Key::Down => game.move_tiles(Direction::Down),
            _ => {}
        }
        write!(
            stdout,
            "{}{}q to quit\r\n{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            game
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Show,
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();
}
