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
        let mut stdout = stdout().into_raw_mode().unwrap();
        Self { stdout }
    }
}

fn main() {
    let tui = Tui::new();
}
