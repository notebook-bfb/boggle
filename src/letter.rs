pub type Letter = u8;

pub fn from_char(ch: char) -> Option<Letter> {
    return match ch {
        'a' => Some(0),
        'b' => Some(1),
        'c' => Some(2),
        'd' => Some(3),
        'e' => Some(4),
        'f' => Some(5),
        'g' => Some(6),
        'h' => Some(7),
        'i' => Some(8),
        'j' => Some(9),
        'k' => Some(10),
        'l' => Some(11),
        'm' => Some(12),
        'n' => Some(13),
        'o' => Some(14),
        'p' => Some(15),
        'q' => Some(16),
        'r' => Some(17),
        's' => Some(18),
        't' => Some(19),
        'u' => Some(20),
        'v' => Some(21),
        'w' => Some(22),
        'x' => Some(23),
        'y' => Some(24),
        'z' => Some(25),
         _  => None
    };
}

pub fn to_char(letter: Letter) -> Option<char> {
    return match letter {
        0 => Some('a'),
        1 => Some('b'),
        2 => Some('c'),
        3 => Some('d'),
        4 => Some('e'),
        5 => Some('f'),
        6 => Some('g'),
        7 => Some('h'),
        8 => Some('i'),
        9 => Some('j'),
        10 => Some('k'),
        11 => Some('l'),
        12 => Some('m'),
        13 => Some('n'),
        14 => Some('o'),
        15 => Some('p'),
        16 => Some('q'),
        17 => Some('r'),
        18 => Some('s'),
        19 => Some('t'),
        20 => Some('u'),
        21 => Some('v'),
        22 => Some('w'),
        23 => Some('x'),
        24 => Some('y'),
        25 => Some('z'),
        _  => None
    };
}

pub fn to_string(word: &Box<[Letter]>) -> Option<String> {
    let mut string = String::new();

    for letter in word.iter() {
        string.push(to_char(*letter)?);
    }

    return Some(string);
}