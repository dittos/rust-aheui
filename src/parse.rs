use cell::*;

fn decompose_hangul(ch: char) -> Option<(u32, u32, u8)> {
    if '가' <= ch && ch <= '힣' {
        let c = ch as u32 - '가' as u32;
        Some((c / 28 / 21, c / 28 % 21, (c % 28) as u8))
    } else {
        None
    }
}

const VALUE_TABLE: [i32; 28] = [ 0, 2, 4, 4, 2, 5, 5, 3, 5, 7, 9, 9, 7, 9, 9, 8, 4, 4, 6, 2, 4, 1, 3, 4, 3, 4, 4, 3 ];

fn parse_op(op: u32, value: u8) -> Op {
    match op {
        2 => Op::Div,
        3 => Op::Add,
        4 => Op::Mul,
        5 => Op::Mod,
        8 => Op::Dup,
        9 => Op::Switch(value),
        10 => Op::Move(value),
        12 => Op::Cmp,
        14 => Op::Branch,
        16 => Op::Sub,
        17 => Op::Swap,
        18 => Op::Exit,
        6 => match value {
            21 => Op::PrintNum,
            27 => Op::PrintChar,
            _ => Op::Pop,
        },
        7 => match value {
            21 => Op::InputNum,
            27 => Op::InputChar,
            _ => Op::Push(VALUE_TABLE[value as usize]),
        },
        _ => Op::Nothing,
    }
}

fn parse_dir(x: u32) -> Dir {
    match x {
        0 => Dir::Set(1, 0),
        2 => Dir::Set(2, 0),
        4 => Dir::Set(-1, 0),
        6 => Dir::Set(-2, 0),
        8 => Dir::Set(0, -1),
        12 => Dir::Set(0, -2),
        13 => Dir::Set(0, 1),
        17 => Dir::Set(0, 2),
        18 => Dir::FlipY,
        19 => Dir::FlipXY,
        20 => Dir::FlipX,
        _ => Dir::Keep,
    }
}

fn parse_char(ch: char) -> Cell {
    match decompose_hangul(ch) {
        Some((first, middle, last)) =>
            Cell { op: parse_op(first, last), dir: parse_dir(middle) },
        None => Cell::empty(),
    }
}

fn parse_line(line: &str) -> Vec<Cell> {
    line.chars().map(parse_char).collect()
}

pub fn parse(code: &str) -> Vec<Vec<Cell>> {
    code.lines().map(parse_line).collect()
}
