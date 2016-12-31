use std::collections::VecDeque;

type StorageId = u8;
type StorageValue = i32;

pub trait Storage {
    fn peek(&self) -> StorageValue;
    fn pop(&mut self) -> StorageValue;
    fn push(&mut self, StorageValue);
    fn swap(&mut self);
    fn dup(&mut self);
    fn size(&self) -> usize;
}

pub struct Stack {
    data: Vec<StorageValue>,
}

impl Stack {
    fn new() -> Stack {
        Stack { data: Vec::new() }
    }
}

impl Storage for Stack {
    fn peek(&self) -> StorageValue {
        match self.data.last() {
            Some(x) => *x,
            None => 0,
        }
    }

    fn pop(&mut self) -> StorageValue {
        self.data.pop().unwrap()
    }

    fn push(&mut self, value: StorageValue) {
        self.data.push(value);
    }

    fn swap(&mut self) {
        let a = self.data.pop().unwrap();
        let b = self.data.pop().unwrap();
        self.data.push(a);
        self.data.push(b);
    }

    fn dup(&mut self) {
        let a = *self.data.last().unwrap();
        self.data.push(a);
    }

    fn size(&self) -> usize {
        return self.data.len();
    }
}

pub struct Queue {
    data: VecDeque<StorageValue>,
}

impl Queue {
    fn new() -> Queue {
        Queue { data: VecDeque::new() }
    }
}

impl Storage for Queue {
    fn peek(&self) -> StorageValue {
        match self.data.front() {
            Some(x) => *x,
            None => 0,
        }
    }

    fn pop(&mut self) -> StorageValue {
        self.data.pop_front().unwrap()
    }

    fn push(&mut self, value: StorageValue) {
        self.data.push_back(value);
    }

    fn swap(&mut self) {
        self.data.swap(0, 1);
    }

    fn dup(&mut self) {
        let a = *self.data.front().unwrap();
        self.data.push_front(a);
    }

    fn size(&self) -> usize {
        return self.data.len();
    }
}

pub struct Mem {
    storages: Vec<Box<Storage>>,
    current_storage_id: StorageId
}

impl Mem {
    pub fn new() -> Mem {
        let mut storages: Vec<Box<Storage>> = Vec::new();
        for id in 0..28 {
            if id == 21 {
                storages.push(Box::new(Queue::new()));
            } else {
                storages.push(Box::new(Stack::new()));
            }
        }
        Mem {
            storages: storages,
            current_storage_id: 0,
        }
    }

    pub fn switch_to(&mut self, id: StorageId) {
        self.current_storage_id = id;
    }

    pub fn push_to(&mut self, id: StorageId, value: StorageValue) {
        self.storages[id as usize].push(value);
    }

    fn current_storage(&self) -> &Box<Storage> {
        &self.storages[self.current_storage_id as usize]
    }

    fn current_storage_mut(&mut self) -> &mut Box<Storage> {
        &mut self.storages[self.current_storage_id as usize]
    }
}

impl Storage for Mem {
    fn peek(&self) -> StorageValue {
        self.current_storage().peek()
    }

    fn pop(&mut self) -> StorageValue {
        self.current_storage_mut().pop()
    }

    fn push(&mut self, value: StorageValue) {
        self.current_storage_mut().push(value)
    }

    fn swap(&mut self) {
        self.current_storage_mut().swap()
    }

    fn dup(&mut self) {
        self.current_storage_mut().dup()
    }

    fn size(&self) -> usize {
        self.current_storage().size()
    }
}
