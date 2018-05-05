use smallvec::SmallVec;
use std::cmp;
use std::fmt;
use std::iter::FromIterator;
use std::mem;
use VecLike;

type Cell = u64;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BitVec {
    v: SmallVec<[Cell; 1]>,
    max_idx: usize,
}

impl VecLike<bool> for BitVec {
    #[inline(always)]
    fn new() -> Self {
        BitVec {
            v: SmallVec::from([0; 1]),
            max_idx: 0,
        }
    }

    #[inline(always)]
    fn with_capacity(n: usize) -> Self {
        BitVec {
            v: SmallVec::from_vec(vec![0; n / Self::BITS_PER_CELL]),
            max_idx: 0,
        }
    }

    fn insert(&mut self, i: usize, v: bool) {
        self.make_space_at(i);
        self.set(i, v);
    }

    fn set(&mut self, i: usize, v: bool) {
        self.max_idx = cmp::max(i, self.max_idx);
        let cell = self.get_cell_mut(i);
        let mask: Cell = 1 << Self::bit_in_cell(i);
        *cell &= !mask;
        *cell |= (v as Cell) << Self::bit_in_cell(i);
    }

    fn fill<I: Iterator<Item = bool>>(&mut self, iter: I) {
        let mut cell_idx = 0;
        let mut value = 0;
        let mut cleared = true;
        for (idx, el) in iter.enumerate() {
            cleared = false;
            if Self::cell_idx(idx) != cell_idx {
                cleared = true;
                *self.get_cell_idx_mut(cell_idx) = value;
                cell_idx += 1;
                value = 0;
            }
            value |= (el as Cell) << Self::bit_in_cell(idx);
            self.max_idx = idx;
        }
        if !cleared {
            *self.get_cell_idx_mut(cell_idx) = value;
        }
    }

    #[inline(always)]
    fn get(&self, i: usize) -> bool {
        (self.get_cell(i) & (1 << Self::bit_in_cell(i))) != 0
    }
}

impl FromIterator<bool> for BitVec {
    fn from_iter<I: IntoIterator<Item = bool>>(v: I) -> BitVec {
        let mut x = BitVec::new();
        x.fill(v.into_iter());
        x
    }
}

impl BitVec {
    const BITS_PER_CELL: usize = mem::size_of::<Cell>() * 8;

    pub fn len(&self) -> usize {
        self.max_idx + 1
    }

    pub fn capacity(&self) -> usize {
        self.v.len() * Self::BITS_PER_CELL
    }

    pub fn iter(&'_ self) -> impl Iterator<Item = bool> + '_ {
        (0..self.len()).map(move |idx| self.get(idx))
    }

    pub fn count_zeros(&self) -> usize {
        self.len() - self.count_ones() as usize
    }

    pub fn count_ones(&self) -> u32 {
        self.v.iter().map(|c| c.count_ones()).sum()
    }

    pub fn clear(&mut self) {
        for cell in self.v.iter_mut() {
            *cell = 0;
        }
    }

    fn get_cell_mut(&mut self, i: usize) -> &mut Cell {
        let index = Self::cell_idx(i);
        self.get_cell_idx_mut(index)
    }

    fn get_cell_idx_mut(&mut self, index: usize) -> &mut Cell {
        while index >= self.v.len() {
            self.v.push(0);
        }
        &mut self.v[index]
    }

    fn get_cell(&self, i: usize) -> Cell {
        let index = Self::cell_idx(i);
        self.v.get(index).cloned().unwrap_or(0)
    }

    fn get_cell_at_idx(&self, index: usize) -> Cell {
        self.v.get(index).cloned().unwrap_or(0)
    }

    #[inline(always)]
    fn cell_idx(i: usize) -> usize {
        i / Self::BITS_PER_CELL
    }

    #[inline(always)]
    fn bit_in_cell(i: usize) -> usize {
        i % Self::BITS_PER_CELL
    }

    fn make_space_at(&mut self, i: usize) {
        let bit = Self::bit_in_cell(i);
        let cell_idx = Self::cell_idx(i);
        let last_bit_in_cell = self.get_cell_at_idx(cell_idx) >> 63 != 0;
        // We don't want to add new cells unless true, but we do want to shift
        // all cells over.  Therefore, if the next cell exists, then we should
        // shift into it.
        if last_bit_in_cell || cell_idx + 2 <= self.v.len() {
            let last_idx = i - bit + (Self::BITS_PER_CELL - 1);
            self.insert(last_idx + 1, last_bit_in_cell);
        }
        let cell = self.get_cell_idx_mut(cell_idx);
        let right_mask = (1 << bit) - 1;
        let right = *cell & right_mask;
        let left = *cell & !right_mask;
        *cell = (left << 1) | right;
    }
}

impl fmt::Debug for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, bits) in self.v.iter().enumerate() {
            for i in (0..8).into_iter().rev() {
                write!(
                    f,
                    "{:08b}",
                    ((bits & (((2 as Cell).pow(8) - 1) << i * 8)) >> i * 8) as u8
                )?;
                if i != 0 {
                    write!(f, ".")?;
                }
            }
            if i + 1 != self.v.len() {
                write!(f, "|")?;
            }
        }
        Ok(())
    }
}
