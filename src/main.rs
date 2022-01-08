use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use termion::raw::RawTerminal;

struct Block {
    x: usize,
    y: usize,
}

struct Snake {
    blocks: Vec<Block>,
}

struct Tui {
    stdout: RawTerminal<std::io::Stdout>,
}

impl Block {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Snake {
    fn new(head: Block) -> Self {
        let head = Block::new(3, 3);
        let blocks = vec![head];
        Self { blocks }
    }
}

impl Tui {
    fn new() -> Self {
        let stdout = stdout().into_raw_mode().unwrap();
        Self { stdout }
    }

    fn draw_snake(&mut self, snake: &Snake) {
        for block in &snake.blocks {
            write!(self.stdout, "{}*", termion::cursor::Goto(block.x as u16, block.y as u16)).unwrap();
        }
    }

    fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
    }
}

fn main() {
    let mut tui = Tui::new();
    let head = Block::new(3, 3);
    let snake = Snake::new(head);
    loop {
        tui.clear();
        tui.draw_snake(&snake);
        let t0 = std::time::Instant::now();
        while std::time::Instant::now() - t0 < std::time::Duration::new(0, 250_000) {

        }
        break;
    }
}
