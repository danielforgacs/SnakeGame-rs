extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Block {
    x: i16,
    y: i16,
}

struct Snake {
    blocks: Vec<Block>,
}

impl Block {
    fn new(x:i16, y: i16) -> Self {
        Self {x, y}
    }
}

impl Snake {
    fn new(x: i16, y: i16) -> Self {
        Self {
            blocks: vec![Block::new(x, y)]
        }
    }

    fn move_snake(&mut self, direction: Direction) {
        let mut x = 0;
        let mut y = 0;
        match direction {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
        let last_block = &self.blocks[self.blocks.len() - 1];
        let block = Block::new(last_block.x + x, last_block.y + y);
        self.blocks.push(block);
    }
}

fn main() {
    // let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut snake = Snake::new(3, 3);

    'main: loop {
        write!(stdout, "{}", termion::clear::All);
        for block in snake.blocks.iter() {
            write!(
                stdout,
                "{}*:{}{}",
                termion::cursor::Goto(block.x as u16, block.y as u16),
                snake.blocks.len(),
                termion::cursor::Hide
            )
                .unwrap();
        }
        stdout.flush().unwrap();
        let mut dir = Direction::Up;
        for c in stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => break 'main,
                Key::Left => dir = Direction::Left,
                Key::Right => dir = Direction::Right,
                Key::Up => dir = Direction::Up,
                Key::Down => dir = Direction::Down,
                _ => {}
            }
            break;
        }
        snake.move_snake(dir);
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
