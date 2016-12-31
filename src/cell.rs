type DirDelta = i8;
type StorageId = u8;
type StorageValue = i32;

#[derive(Debug)]
#[derive(Clone)]
pub enum Dir {
    Set(DirDelta, DirDelta),
    FlipX,
    FlipY,
    FlipXY,
    Keep,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Op {
    Div,
    Add,
    Mul,
    Mod,
    Dup,
    Switch(StorageId),
    Move(StorageId),
    Cmp,
    Branch,
    Sub,
    Swap,
    Exit,
    PrintNum,
    PrintChar,
    Pop,
    InputNum,
    InputChar,
    Push(StorageValue),
    Nothing,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Cell {
    pub op: Op,
    pub dir: Dir,
}

impl Cell {
    pub fn empty() -> Cell {
        Cell {
            op: Op::Nothing,
            dir: Dir::Keep,
        }
    }
}
