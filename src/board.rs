use crate::letter::{self, Letter};

pub struct Board {
    buffer: [Letter; 16]
}

impl Board {
    pub fn from(string: &str) -> Option<Board> {
        if string.len() != 16 {
            return None;
        }

        let mut buffer = [0 as u8; 16];
        let mut i = 0;

        for ch in string.chars() {
            buffer[i] = letter::from_char(ch)?;
            i += 1;
        }

        return Some(Board {
            buffer: buffer
        });
    }

    pub fn get(&self, x: i8, y: i8) -> Option<Letter> {
        if x < 0 || x > 3 || y < 0 || y > 3 {
            return None;
        }

        return Some(self.buffer[(x + 4 * y) as usize]);
    }

    pub fn iter(&self) -> BoardIterator<'_> {
        return BoardIterator::<'_> {
            board: &self,
            index: 0
        };
    }
}

pub struct BoardIterator<'a> {
    board: &'a Board,
    index: i8
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = (i8, i8, Letter);

    fn next(&mut self) -> Option<(i8, i8, Letter)> {
        if self.index == 16 {
            return None;
        }

        let item: Self::Item = (self.index % 4, self.index / 4, self.board.buffer[self.index as usize]);
        self.index += 1;

        return Some(item);
    }
}