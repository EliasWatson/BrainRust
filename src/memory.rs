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

    pub fn move_index(&mut self, offset: isize) {
        self.index = self.offset_index(offset);
    }

    pub fn add(&mut self, n: u8) {
        self.data[self.index] = self.data[self.index].wrapping_add(n);
    }

    pub fn add_with_offset(&mut self, n: u8, offset: isize) {
        let index = self.offset_index(offset);
        self.data[index] = self.data[index].wrapping_add(n);
    }

    pub fn zero(&mut self) {
        self.data[self.index] = 0;
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

    fn offset_index(&self, offset: isize) -> usize {
        if offset >= 0 {
            (self.index + offset as usize) % self.data.len()
        } else {
            let negative_offset = (-offset) as usize;

            if negative_offset > self.index {
                self.data.len() - (negative_offset - self.index)
            } else {
                self.index - negative_offset
            }
        }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}
