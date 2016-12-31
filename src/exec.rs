use std::io;
use std::char;
use std::process::exit;
use cell::*;
use space::Space;
use mem::{Mem, Storage};

fn required_elems(op: &Op) -> usize {
    match *op {
        Op::Div => 2,
        Op::Add => 2,
        Op::Mul => 2,
        Op::Mod => 2,
        Op::Dup => 1,
        Op::Move(_) => 1,
        Op::Cmp => 2,
        Op::Branch => 1,
        Op::Sub => 2,
        Op::Swap => 2,
        Op::PrintNum => 1,
        Op::PrintChar => 1,
        Op::Pop => 1,
        _ => 0,
    }
}

fn stack2<F>(mem: &mut Mem, func: F)
    where F: Fn(i32, i32) -> i32 {
    let x = mem.pop();
    let y = mem.pop();
    mem.push(func(y, x));
}

fn execute_op(op: &Op, mem: &mut Mem, writer: &mut io::Write) {
    match *op {
        Op::InputNum => { /* TODO */ },
        Op::InputChar => { /* TODO */ },
        Op::PrintNum => write!(writer, "{}", mem.pop()).unwrap(),
        Op::PrintChar => write!(writer, "{}", char::from_u32(mem.pop() as u32).unwrap()).unwrap(),
        Op::Exit => exit(mem.peek()),
        Op::Div => stack2(mem, |x, y| x / y),
        Op::Add => stack2(mem, |x, y| x + y),
        Op::Mul => stack2(mem, |x, y| x * y),
        Op::Mod => stack2(mem, |x, y| x % y),
        Op::Dup => mem.dup(),
        Op::Switch(i) => mem.switch_to(i),
        Op::Move(i) => { let x = mem.pop(); mem.push_to(i, x) },
        Op::Cmp => stack2(mem, |y, x| if y >= x { 1 } else { 0 }),
        Op::Sub => stack2(mem, |x, y| x - y),
        Op::Swap => mem.swap(),
        Op::Pop => { mem.pop(); },
        Op::Push(x) => mem.push(x),
        Op::Branch => { mem.pop(); },
        Op::Nothing => {},
    }
}

fn wrap(pos: i32, size: i32) -> i32 {
    if pos < 0 {
        (size - 1) as i32
    } else if pos >= size {
        0
    } else {
        pos
    }
}

pub fn execute(space: Space) {
    let mut mem = Mem::new();
    let mut x = 0;
    let mut y = 0;
    let mut dx: i32 = 0;
    let mut dy: i32 = 1;
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    loop {
        let cell = space.cell_at(x, y);
        let branch = match cell.op {
            Op::Branch => mem.peek() == 0,
            _ => false,
        };
        let underflow = mem.size() < required_elems(&cell.op);
        if !underflow {
            execute_op(&cell.op, &mut mem, &mut writer);
        };

        let (dx2, dy2) = match cell.dir {
            Dir::Set(sx, sy) => (sx as i32, sy as i32),
            Dir::FlipX => (-dx, dy),
            Dir::FlipY => (dx, -dy),
            Dir::FlipXY => (-dx, -dy),
            Dir::Keep => (dx, dy),
        };
        dx = dx2;
        dy = dy2;
        if underflow {
            dx = -dx;
            dy = -dy;
        }
        if branch {
            dx = -dx;
            dy = -dy;
        }
        x = wrap(x + dx, space.width as i32);
        y = wrap(y + dy, space.height as i32);
    }
}
