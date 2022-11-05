use crate::{
    board::Board,
    tree::{
        Node,
        WordTree
    },
    letter::Letter
};

struct Worker {
    word: Box<[Letter]>,
    node: Node,
    x: i8,
    y: i8
}

struct Info {
    workers: Vec<Worker>,
    answers: Vec<String>
}

pub fn solve(board: &Board, tree: &WordTree) -> Box<[String]> {
    let mut info = Info {
        workers: Vec::<Worker>::with_capacity(16),
        answers: Vec::<String>::new()
    };

    for (x, y, letter) in board.iter() {
        info.workers.push(Worker {
            word: Box::new([]),
            node: WordTree::root(),
            x: x,
            y: y
        });
    }

    loop {
        let worker = match info.workers.pop() {
            Some(worker) => worker,
            None => {
                break;
            }
        };

        //TODO: finish implementing answer searching
    }

    return info.answers.into_boxed_slice();
}