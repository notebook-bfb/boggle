use crate::util;

pub struct Node {
    is_word: bool,
    children: [u32; 26]
}

pub struct WordTreeGenerator {
    nodes: Vec<Node>
}

impl WordTreeGenerator {
    pub fn new() -> WordTreeGenerator {
        let mut generator = WordTreeGenerator {
            nodes: Vec::<Node>::new()
        };

        generator.nodes.push(Node {
            is_word: false,
            children: [u32::MAX; 26]
        });

        return generator;
    }

    pub fn add(&mut self, word: &str) -> bool {
        let mut using = Vec::<usize>::new();
        let mut index: usize = 0;
        for ch in word.chars() {
            let offset = match util::as_offset(ch) {
                Some(o) => o as usize,
                None => {
                    return false;
                }
            };

            if self.nodes[index].children[offset] == u32::MAX {
                self.nodes[index].children[offset] = self.nodes.len() as u32;

                self.nodes.push(Node {
                    is_word: false,
                    children: [u32::MAX; 26]
                });
            }

            index = self.nodes[index].children[offset] as usize;
            using.push(index);
        }
        self.nodes[index].is_word = true;

        return true;
    }

    pub fn serialize(&self) -> Box<[u32]> {
        //TODO: does this REALLY require a vector?
        let mut indices: Box<[u32]> = vec![0; self.nodes.len()].into_boxed_slice();
        let mut buffer: Box<[u32]> = vec![0; self.nodes.len() * 2 - 1].into_boxed_slice();

        let mut index: usize = 0;
        for i in 0..self.nodes.len() {
            indices[i] = index as u32;

            let mut flags: u32 = 0;
            let mut offset = 1;

            for j in 0..26 {
                if self.nodes[i].children[j] != u32::MAX {
                    flags |= 1 << j;
                    offset += 1;
                }
            }

            if self.nodes[i].is_word {
                flags |= 1 << 26;
            }

            buffer[index as usize] = flags;
            index += offset;
        }

        for i in 0..self.nodes.len() {
            let mut index = indices[i];

            for id in self.nodes[i].children {
                if id != u32::MAX {
                    index += 1;
                    buffer[index as usize] = indices[id as usize];
                }
            }
        }

        return buffer;
    }
}