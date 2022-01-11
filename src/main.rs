use std::collections::VecDeque;
use std::io::{Write, stdout, stdin};
use rand::prelude::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Block {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct Snake {
    blocks: VecDeque<Block>,
    direction: Direction,
}

#[derive(Debug)]
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
        let head = Block::new(x, y);
        let blocks2 = VecDeque::from_iter([head]);
        Self {
            blocks: blocks2,
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
        let last_block = self.blocks.front().unwrap();
        let block = Block::new(last_block.x + x, last_block.y + y);

        if food.block == block {
            loop {
                food.block.x = thread_rng().gen_range(2..15);
                food.block.y = thread_rng().gen_range(2..15);
                if !self.check_block_snake_colision(&food.block) {
                    break;
                }
            }
        } else {
            self.blocks.pop_back();
        }
        self.blocks.push_front(block);
        self.direction = direction;
    }

    fn check_block_snake_colision(&self, block: &Block) -> bool {
        let mut collision = false;
        for snake_block in &self.blocks {
            match block == snake_block {
                false => (),
                true => {
                    collision = true;
                    break;
                },
            };
        }
        collision
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
    let mut food = Food::new(7, 2);

    'main: loop {
        write!(stdout, "{}", termion::clear::All).unwrap();
        for block in snake.blocks.iter() {
            write!(stdout, "{}#", termion::cursor::Goto(food.block.x as u16, food.block.y as u16)).unwrap();
            write!(
                stdout,
                "{}*{}",
                termion::cursor::Goto(block.x as u16, block.y as u16),
                termion::cursor::Hide
            )
                .unwrap();
        }
        write!(stdout,
            "{}len: {}",
            termion::cursor::Goto(1, 20),
            snake.blocks.len(),
        ).unwrap();
        write!(stdout,
            "{}{:?}\n\r{:?}\n\rfood: {:?}",
            termion::cursor::Goto(1, 21),
            snake.blocks,
            snake.blocks,
            food,
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
