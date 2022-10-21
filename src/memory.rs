use std::fmt::Display;

#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
    index: usize,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            index: 0,
        }
    }

    pub fn next(&mut self, n: usize) {
        self.index = (self.index + n) % self.data.len();
    }

    pub fn previous(&mut self, n: usize) {
        self.index = if n > self.index {
            self.data.len() - (n - self.index)
        } else {
            self.index - n
        };
    }

    pub fn increment(&mut self, n: u8) {
        self.data[self.index] = self.data[self.index].wrapping_add(n);
    }

    pub fn decrement(&mut self, n: u8) {
        self.data[self.index] = self.data[self.index].wrapping_sub(n);
    }

    pub fn is_zero(&self) -> bool {
        self.data[self.index] == 0
    }

    pub fn get_char(&self) -> char {
        self.data[self.index] as char
    }

    pub fn set_char(&mut self, c: char) {
        self.data[self.index] = c as u8;
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}
