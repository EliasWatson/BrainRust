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

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.data.len();
    }

    pub fn previous(&mut self) {
        self.index = if self.index == 0 {
            self.data.len() - 1
        } else {
            self.index - 1
        };
    }

    pub fn increment(&mut self) {
        self.data[self.index] = self.data[self.index].wrapping_add(1);
    }

    pub fn decrement(&mut self) {
        self.data[self.index] = self.data[self.index].wrapping_sub(1);
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
