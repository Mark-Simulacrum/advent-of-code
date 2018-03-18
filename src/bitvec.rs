use smallvec::SmallVec;
use std::mem;
use std::fmt;
use VecLike;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BitVec(SmallVec<[u64; 1]>);

impl VecLike<bool> for BitVec {
    #[inline(always)]
    fn new() -> Self {
        BitVec(SmallVec::from([0; 1]))
    }

    #[inline(always)]
    fn with_capacity(n: usize) -> Self {
        BitVec(SmallVec::from_vec(vec![0; n / Self::BITS_PER_CELL]))
    }

    fn insert(&mut self, i: usize, v: bool) {
        self.make_space_at(i);
        self.set(i, v);
    }

    fn set(&mut self, i: usize, v: bool) {
        let cell = self.get_cell_mut(i);
        let mask = 1u64 << Self::bit_in_cell(i);
        *cell &= !mask;
        *cell |= (v as u64) << Self::bit_in_cell(i);
    }

    fn fill<I: Iterator<Item = bool>>(&mut self, iter: I) {
        let mut cell_idx = 0;
        let mut value = 0;
        let mut cleared = true;
        for (idx, el) in iter.enumerate() {
            cleared = false;
            value |= (el as u64) << Self::bit_in_cell(idx);
            if Self::cell_idx(idx) != cell_idx {
                cleared = true;
                *self.get_cell_idx_mut(cell_idx) = value;
                cell_idx += 1;
                value = 0;
            }
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

impl BitVec {
    const BITS_PER_CELL: usize = mem::size_of::<u64>() * 8;
    pub fn count_ones(&self) -> u32 {
        self.0.iter().map(|c| c.count_ones()).sum()
    }

    fn get_cell_mut(&mut self, i: usize) -> &mut u64 {
        let index = Self::cell_idx(i);
        self.get_cell_idx_mut(index)
    }

    fn get_cell_idx_mut(&mut self, index: usize) -> &mut u64 {
        while index >= self.0.len() {
            self.0.push(0);
        }
        &mut self.0[index]
    }

    fn get_cell(&self, i: usize) -> u64 {
        let index = Self::cell_idx(i);
        self.0.get(index).cloned().unwrap_or(0)
    }

    fn get_cell_at_idx(&self, index: usize) -> u64 {
        self.0.get(index).cloned().unwrap_or(0)
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
        if last_bit_in_cell || cell_idx + 2 <= self.0.len() {
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
        for (i, bits) in self.0.iter().enumerate() {
            for i in (0..8).into_iter().rev() {
                write!(
                    f,
                    "{:08b}",
                    ((bits & ((2u64.pow(8) - 1) << i * 8)) >> i * 8) as u8
                )?;
                if i != 0 {
                    write!(f, ".")?;
                }
            }
            if i + 1 != self.0.len() {
                write!(f, "|")?;
            }
        }
        Ok(())
    }
}
