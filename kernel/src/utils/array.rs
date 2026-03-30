use core::ops::Index;

pub(crate) struct PositionalSequence<T, const CAP: usize>{
    list: [Option<T>; CAP],
    _cursor: usize,
    count: usize,
}

impl<T, const CAP: usize> PositionalSequence<T, CAP>{
    pub fn new() -> Self {
        Self {
            list: [(); CAP].map(|_| None),
            _cursor: 0,
            count: CAP,
        }
    }

    pub fn pos(&mut self, index: usize){
        if index >= self.count {
            panic!("Index out of bounds");
        }
        self._cursor = index;
    }

    pub fn insert(&mut self, item: T){
        self.list[self._cursor] = Some(item);
        self._cursor += 1;
    }

    pub fn remove(&mut self){
        self._cursor -= 1;
        self.list[self._cursor] = None;
    }

    pub fn current(&self) -> &Option<T>{
        self.get(self._cursor - 1)
    }

    fn get(&self, index: usize) -> &Option<T>{
        if index < self.count {
            &self.list[index]
        } else {
            panic!("Index out of bounds");
        }
    }
}

impl<T, const CAP: usize> Index<usize> for PositionalSequence<T, CAP>{
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.get(index)
    }
}