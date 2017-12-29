use std::fmt;
use {VecLike, BitVec};
use smallvec::SmallVec;
use std::marker::PhantomData;

const ON: u8 = b'#';
const OFF: u8 = b'.';

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T, C> {
    left_bound: isize,
    top_bound: isize,
    matrix: Matrix<T, C>,
}

impl<T: Copy + Default, C: VecLike<T>> Grid<T, C> {
    pub fn new() -> Self {
        Grid {
            matrix: Matrix::new(3, 3),
            left_bound: -1,
            top_bound: -1,
        }
    }

    pub fn from_matrix(matrix: Matrix<T, C>, top_left: (isize, isize)) -> Self {
        Grid {
            matrix,
            left_bound: top_left.0,
            top_bound: top_left.1,
        }
    }

    pub fn to_matrix(&mut self, x: isize, y: isize) -> (usize, usize) {
        while x < self.left_bound {
            self.matrix.insert_column_left();
            self.left_bound -= 1;
        }
        while y < self.top_bound {
            self.matrix.insert_row_top();
            self.top_bound -= 1;
        }
        let col = (x - self.left_bound) as usize;
        let row = (y - self.top_bound) as usize;
        (row, col)
    }

    pub fn get(&mut self, x: isize, y: isize) -> T {
        let (row, col) = self.to_matrix(x, y);
        self.matrix.get(row, col).unwrap_or_default()
    }

    pub fn set(&mut self, x: isize, y: isize, v: T) {
        let (row, col) = self.to_matrix(x, y);
        self.matrix.set(row, col, v);
    }
}

impl Grid<bool, BitVec> {
    pub fn count_set(&self) -> u32 {
        self.matrix.count_set()
    }
}

impl<C: VecLike<bool>> fmt::Debug for Grid<bool, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", DebugMatrix(&self.matrix, "\n"))
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix<T, C> {
    matrix: C,
    rows: usize,
    cols: usize,
    _d: PhantomData<T>,
}

impl Matrix<bool, BitVec> {
    pub fn count_set(&self) -> u32 {
        self.matrix.count_ones()
    }
}

impl<T, C> Matrix<T, C>
where
    T: Copy + Default,
    C: VecLike<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            matrix: C::with_capacity(rows * cols),
            rows: rows,
            cols: cols,
            _d: PhantomData,
        }
    }

    pub fn interpret(pattern: &str, empty: T, present: T) -> Self {
        let mut matrix = C::new();
        let mut cols = 0;
        let mut rows = 1;
        let mut idx = 0;
        for (i, &b) in pattern.trim().as_bytes().iter().enumerate() {
            if b == b'/' || b == b'\n' {
                if cols == 0 {
                    cols = i;
                }
                rows += 1;
                continue;
            }
            assert!(b == ON || b == OFF, "unexpected byte {:?} in {:?}", b as char, pattern);
            if b == ON {
                matrix.set(idx, present);
            } else {
                matrix.set(idx, empty);
            }
            idx += 1;
        }
        assert_ne!(cols, 0);
        Matrix {
            matrix,
            cols,
            rows,
            _d: PhantomData,
        }
    }

    fn insert_row_top(&mut self) {
        for _ in 0..self.cols {
            self.matrix.insert(0, T::default());
        }
        self.rows += 1;
    }

    fn insert_row_bottom(&mut self) {
        let start = (self.rows + 1) * self.cols;
        for i in 0..self.cols {
            self.matrix.set(start + i, T::default());
        }
        self.rows += 1;
    }

    fn insert_column_left(&mut self) {
        let mut inserted = 0;
        for row in 0..self.rows {
            self.matrix.insert(row * self.cols + inserted, T::default());
            inserted += 1;
        }
        self.cols += 1;
    }

    fn insert_column_right(&mut self) {
        let mut inserted = 0;
        for row in 0..self.rows {
            let idx = row*self.cols + self.cols + inserted;
            inserted += 1;
            self.matrix.insert(idx, T::default());
        }
        self.cols += 1;
    }

    pub fn columns(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        if col < self.cols && row < self.rows {
            Some(self.matrix.get(row*self.cols + col))
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, v: T) {
        while row >= self.rows {
            self.insert_row_bottom();
        }
        while col >= self.cols {
            self.insert_column_right();
        }
        assert!(col <= self.cols);
        assert!(row <= self.rows);
        self.matrix.set(row*self.cols + col, v);
    }

    pub fn transpose(&mut self) {
        let mut matrix = Matrix::new(self.rows, self.cols);
        for row in 0..self.rows {
            let x = (0..self.cols)
                .into_iter()
                .map(|c| self.get(row, c).unwrap())
                .collect::<SmallVec<[_; 5]>>();
            for (col, v) in x.into_iter().enumerate() {
                matrix.set(col, row, v);
            }
        }
        *self = matrix;
    }

    pub fn reverse_rows(&mut self) {
        for row in 0..self.rows {
            let x = (0..self.cols)
                .into_iter()
                .rev()
                .map(|c| self.get(row, c).unwrap())
                .collect::<SmallVec<[_; 5]>>();
            for (i, v) in x.into_iter().enumerate() {
                self.set(row, i, v);
            }
        }
    }

    pub fn rotate_90(&mut self) {
        self.transpose();
        self.reverse_rows();
    }

    pub fn reversed_rows(&self) -> Self {
        let mut a = self.clone();
        a.reverse_rows();
        a
    }

    // This loads from the other matrix, using our own indices as a basis for
    // how far to iterate.
    pub fn load_from(&mut self, start: usize, other: &Self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let o = other.matrix.get(start + row*other.cols + col);
                self.set(row, col, o);
            }
        }
    }

    // Loads from the other matrix, using it's indices to fill our own.  N.B.
    // The difference from load_from is that we offset `start` into our matrix
    // instead of the other matrix.
    pub fn set_from(&mut self, start: usize, other: &Self) {
        for row in 0..other.rows {
            for col in 0..other.cols {
                let o = other.matrix.get(row*other.cols + col);
                let idx = start + row*self.cols + col;
                self.matrix.set(idx, o);
            }
        }
    }
}

#[test]
fn matrix_interpret() {
    let mut matrix = Matrix::new(2, 2);
    matrix.set(0, 1, true);
    matrix.set(1, 1, true);
    assert_eq!(Matrix::interpret(".#/.#"), matrix);
}

#[test]
fn matrix_reverse_rows() {
    let mut input = Matrix::interpret(".#/.#");
    input.reverse_rows();
    assert_eq!(input, Matrix::interpret("#./#."));
}

#[test]
fn matrix_transpose() {
    let mut input = Matrix::interpret(".#/.#");
    input.transpose();
    assert_eq!(input, Matrix::interpret("../##"));
}

#[test]
fn matrix_rotate90() {
    let mut input = Matrix::interpret(".#/.#");
    input.rotate_90();
    assert_eq!(input, Matrix::interpret("../##"));
}

struct DebugMatrix<'a, 'b, T: 'a, C: 'a>(&'a Matrix<T, C>, &'b str);

impl<C: VecLike<bool>> fmt::Debug for Matrix<bool, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:?}", DebugMatrix(self, "\n"))
        } else {
            write!(f, "{:?}", DebugMatrix(self, "/"))
        }
    }
}

impl<'a, 'b, C: VecLike<bool>> fmt::Debug for DebugMatrix<'a, 'b, bool, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.0.rows {
            for j in 0..self.0.cols {
                let idx = i*self.0.cols + j;
                if self.0.matrix.get(idx) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            if i + 1 != self.0.rows {
                write!(f, "{}", self.1)?;
            }
        }
        Ok(())
    }
}

#[test]
fn matrix_dbg_1() {
    assert_eq!(format!("{:?}", Matrix::interpret("../..")), "../..");
}

#[test]
fn matrix_dbg_2() {
    assert_eq!(format!("{:?}", Matrix::interpret(".#/#.")), ".#/#.");
}

#[test]
fn matrix_dbg_3() {
    assert_eq!(format!("{:?}", Matrix::interpret(".#./..#/...")), ".#./..#/...");
}

#[test]
fn matrix_insert_left() {
    let mut m = Matrix::interpret(".#/##");
    m.insert_column_left();
    assert_eq!(m, Matrix::interpret("..#/.##"));
}

#[test]
fn matrix_insert_left_1() {
    let mut m = Matrix::interpret("
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
");
    m.insert_column_left();
    assert_eq!(m, Matrix::interpret("
.....#..#.
.....#..#.
.....#..#.
.....#..#.
.....#..#.
.....#..#.
.....#..#.
.....#..#.
.....#..#.
.....#..#.
.....#..#.
.....#..#.
"));
}


#[test]
fn matrix_insert_right() {
    let mut m = Matrix::interpret(".#/##");
    m.insert_column_right();
    assert_eq!(m, Matrix::interpret(".#./##."));
}

#[test]
fn matrix_insert_right_1() {
    let mut m = Matrix::interpret("
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
");
    m.insert_column_right();
    assert_eq!(m, Matrix::interpret("
....#..#..
....#..#..
....#..#..
....#..#..
....#..#..
....#..#..
....#..#..
....#..#..
....#..#..
....#..#..
....#..#..
....#..#..
"));
}

#[test]
fn matrix_insert_row_top() {
    let mut m = Matrix::interpret(".#/##");
    m.insert_row_top();
    assert_eq!(m, Matrix::interpret("../.#/##"));
}

#[test]
fn matrix_insert_row_top_1() {
    let mut m = Matrix::interpret("
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
");
    m.insert_row_top();
    assert_eq!(m, Matrix::interpret("
.........
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
....#..#.
"));
}

