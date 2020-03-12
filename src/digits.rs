use std::io::prelude::*;
use std::io::Stdout;
use termion::raw::RawTerminal;

pub trait Digit {
    fn render(&self, position: (u16, u16), raw_stdout: &mut RawTerminal<Stdout>);
}

pub struct Zero;
pub struct One;
pub struct Two;
pub struct Three;
pub struct Four;
pub struct Five;
pub struct Six;
pub struct Seven;
pub struct Eight;
pub struct Nine;

impl Digit for Zero {
    fn render(&self, position: (u16, u16), raw_stdout: &mut RawTerminal<Stdout>) {
        let layers = vec![" 0000", "00  00", "00  00", "00  00", " 0000"];
        render(position, layers, raw_stdout);
    }
}

impl Digit for One {
    fn render(&self, position: (u16, u16), raw_stdout: &mut RawTerminal<Stdout>) {
        let layers = vec!["1111", "  11", "  11", "  11", "111111"];
        render(position, layers, raw_stdout);
    }
}

impl Digit for Two {
    fn render(&self, position: (u16, u16), raw_stdout: &mut RawTerminal<Stdout>) {
        let layers = vec![" 2222", "22  22", "   22", "  22", "222222"];
        render(position, layers, raw_stdout);
    }
}

impl Digit for Three {
    fn render(&self, position: (u16, u16), raw_stdout: &mut RawTerminal<Stdout>) {
        let layers = vec![" 3333", "33  33", "   333", "33  33", " 3333"];
        render(position, layers, raw_stdout);
    }
}

impl Digit for Four {
    fn render(&self, position: (u16, u16), raw_stdout: &mut RawTerminal<Stdout>) {
        let layers = vec!["44  44", "44  44", "444444", "    44", "    44"];
        render(position, layers, raw_stdout);
    }
}

impl Digit for Five {
    fn render(&self, position: (u16, u16), raw_stdout: &mut RawTerminal<Stdout>) {
        let layers = vec!["555555", "55", "555555", "    55", "55555"];
        render(position, layers, raw_stdout);
    }
}

impl Digit for Six {
    fn render(&self, position: (u16, u16), raw_stdout: &mut RawTerminal<Stdout>) {
        let layers = vec![" 6666", "66", "66666", "66  66", " 6666"];
        render(position, layers, raw_stdout);
    }
}

impl Digit for Seven {
    fn render(&self, position: (u16, u16), raw_stdout: &mut RawTerminal<Stdout>) {
        let layers = vec!["777777", "    77", "   77", "  77", "77"];
        render(position, layers, raw_stdout);
    }
}

impl Digit for Eight {
    fn render(&self, position: (u16, u16), raw_stdout: &mut RawTerminal<Stdout>) {
        let layers = vec![" 8888", "88  88", " 8888", "88  88", " 8888"];
        render(position, layers, raw_stdout);
    }
}

impl Digit for Nine {
    fn render(&self, position: (u16, u16), raw_stdout: &mut RawTerminal<Stdout>) {
        let layers = vec![" 9999", "99  99", " 99999", "    99", " 9999"];
        render(position, layers, raw_stdout);
    }
}

// Helper functtion that renders layesr given
// Useful since all digits are composed of 5 layers
fn render(position: (u16, u16), layers: Vec<&str>, raw_stdout: &mut RawTerminal<Stdout>) {
    let (x, mut y) = position;
    for layer in layers {
        write!(raw_stdout, "{}{}", termion::cursor::Goto(x, y), layer)
            .expect("Failed to write digit to screen");
        y += 1;
    }
    raw_stdout.flush().unwrap();
}
