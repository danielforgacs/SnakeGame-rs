extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

#[derive(Clone, Copy)]
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
    direction: Direction,
}

impl Block {
    fn new(x:i16, y: i16) -> Self {
        Self {x, y}
    }
}

impl Snake {
    fn new(x: i16, y: i16, direction: Direction) -> Self {
        Self {
            blocks: vec![Block::new(x, y)],
            direction,
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
        self.direction = direction;
    }
}

fn main() {
    // let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut snake = Snake::new(3, 3, Direction::Right);

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
        // let mut dir = &snake.direction;
        for c in stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => break 'main,
                Key::Left => snake.move_snake(Direction::Left),
                Key::Right => snake.move_snake(Direction::Right),
                Key::Up => snake.move_snake(Direction::Up),
                Key::Down => snake.move_snake(Direction::Down),
                Key::Char('m') => snake.move_snake(snake.direction),
                _ => {}
            }
            break;
        }
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
