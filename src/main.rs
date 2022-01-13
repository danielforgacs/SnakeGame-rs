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

struct Display {
    blocks: Vec<Block>,
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

    fn check_sneak_head_blocks_clossion<'a, T: Iterator<Item = &'a Block>>(&self, blocks: T) -> bool {
        let head = self.blocks.front().unwrap();
        let mut collision = false;
        for block in blocks {
            if block.x == head.x - 1 && block.y == head.y -1 {
                collision = true;
                break;
            }
        }
        collision
    }
}

impl Food {
    fn new(x: i16, y: i16) -> Self {
        Self { block: Block::new(x, y) }
    }
}

impl Display {
    fn new(w: u8, h: u8) -> Self {
        let w = w as i16;
        let h = h as i16;
        let mut blocks = Vec::<Block>::new();
        for i in 0..2_i16 {
            for k in 0..w as i16 {
                let block = Block::new(k, i * h);
                blocks.push(block);
            }
            for l in 0..h as i16 {
                let block = Block::new(i * w, l);
                blocks.push(block);
            }
        }
        Self { blocks }
    }
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut snake = Snake::new(3, 3, Direction::Right);
    let mut food = Food::new(7, 2);
    let display = Display::new(50, 25);

    'main: loop {
        write!(stdout, "{}", termion::clear::All).unwrap();
        for block in &display.blocks {
            write!(stdout, "{}+", termion::cursor::Goto(block.x as u16 + 1, block.y as u16 + 1)).unwrap();
        }
        write!(stdout, "{}#", termion::cursor::Goto(food.block.x as u16, food.block.y as u16)).unwrap();
        for block in snake.blocks.iter() {
            write!(stdout, "{}*{}", termion::cursor::Goto(block.x as u16, block.y as u16), termion::cursor::Hide ).unwrap();
        }
        stdout.flush().unwrap();
        let old_dir = snake.direction;
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
        if snake.blocks.len() > 1 {
            match (old_dir, snake.direction) {
                (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => break 'main,
                (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => break 'main,
                _ => (),
            }
        }
        if snake.check_sneak_head_blocks_clossion(display.blocks.iter()) {
            break;
        }
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
