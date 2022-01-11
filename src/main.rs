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

struct Food {
    block: Block,
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

    fn move_snake(&mut self, direction: Direction, food: &mut Food) {
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
        // assert_eq!(food.block.x, 7);
        // let xyz: () = food.block.x;

        if food.block.x == block.x && food.block.y == block.y {
            food.block.x = 10;
            food.block.y = 10;
            // panic!("FUND");
        } else {

            self.blocks.remove(self.blocks.len() - 1);
        }
        self.blocks.push(block);
        self.direction = direction;
    }
}

impl Food {
    fn new(x: i16, y: i16) -> Self {
        Self { block: Block::new(x, y) }
    }
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut snake = Snake::new(3, 3, Direction::Right);
    // let food = Block::new(7, 2);
    let mut food = Food::new(7, 2);

    // let block0 = Block::new(4, 3);
    // let block1 = Block::new(5, 3);
    // snake.blocks.push(block0);
    // snake.blocks.push(block1);

    'main: loop {
        write!(stdout, "{}", termion::clear::All).unwrap();
        for block in snake.blocks.iter() {
            write!(stdout, "{}#", termion::cursor::Goto(food.block.x as u16, food.block.y as u16)).unwrap();
            write!(
                stdout,
                "{}*{}",
                termion::cursor::Goto(block.x as u16, block.y as u16),
                // snake.blocks.len(),
                termion::cursor::Hide
            )
                .unwrap();
        }
        write!(stdout,
            "{}len: {}",
            termion::cursor::Goto(1, 20),
            snake.blocks.len(),
        ).unwrap();
        stdout.flush().unwrap();
        for c in stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => break 'main,
                Key::Left => snake.move_snake(Direction::Left, &mut food),
                Key::Right => snake.move_snake(Direction::Right, &mut food),
                Key::Up => snake.move_snake(Direction::Up, &mut food),
                Key::Down => snake.move_snake(Direction::Down, &mut food),
                Key::Char('m') => snake.move_snake(snake.direction, &mut food),
                _ => {}
            }
            break;
        }
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
