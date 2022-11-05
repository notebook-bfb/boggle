use crate::letter::{self, Letter};

pub struct WordTree {
    buffer: Box<[u32]>,
}

#[derive(Copy, Clone)]
pub struct Node {
    id: u32
}

impl WordTree {
    pub fn new(buffer: Box<[u32]>) -> WordTree {
        return WordTree {
            buffer: buffer
        };
    }

    pub fn root() -> Node {
        return Node {
            id: 0
        };
    }

    pub fn is_word(&self, node: Node) -> bool {
        return (self.buffer[node.id as usize] >> 26) & 1 == 1;
    }

    pub fn get(&self, node: Node, letter: Letter) -> Option<Node> {
        let flags = self.buffer[node.id as usize];
        if flags == 0 || flags == (1 << 26) {
            return None;
        }

        let flag = 1 << letter;
        if flags & flag == 0 {
            return None;
        }
        
        let mut index = node.id;
        let mut mask = 1;
        for _ in 0..26 {
            if flags & mask != 0 {
                index += 1;
            }

            if mask & flag != 0 {
                return Some(Node { id: self.buffer[index as usize] });
            }

            mask <<= 1;
        }

        return None;
    }
}