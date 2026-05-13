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


struct Matrix<T, const X: usize, const Y: usize> {
    data: [[T; X]; Y],
    rows: usize,
    cols: usize,
}

impl<T: Clone + Default, const X: usize, const Y: usize> Matrix<T, X, Y> {
    /// Crée une nouvelle matrice de taille `rows x cols` avec des valeurs par défaut.
    pub fn new() -> Self {
        Matrix {
            data: [(); Y].map(|_| [(); X].map(|_| T::default())),
            rows: Y,
            cols: X,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row][col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> bool {
        if row < self.rows && col < self.cols {
            self.data[row][col] = value;
            true
        } else {
            false
        }
    }
}