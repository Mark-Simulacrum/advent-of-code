use smallvec::SmallVec;
use std::mem;
use std::fmt;
use VecLike;

#[derive(Clone, PartialEq, Eq)]
pub struct BitVec(SmallVec<[u64; 1]>);

impl VecLike<bool> for BitVec {
    fn new() -> Self {
        BitVec(SmallVec::from([0; 1]))
    }

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
        if v {
            *cell |= mask;
        }
    }

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
        let index = i / Self::BITS_PER_CELL;
        while index >= self.0.len() { self.0.push(0); }
        &mut self.0[index]
    }

    fn get_cell(&self, i: usize) -> u64 {
        let index = i / Self::BITS_PER_CELL;
        self.0.get(index).cloned().unwrap_or(0)
    }

    fn bit_in_cell(i: usize) -> usize {
        i % Self::BITS_PER_CELL
    }

    fn make_space_at(&mut self, i: usize) {
        let bit = Self::bit_in_cell(i);
        let last_bit_in_cell = self.get_cell(i) >> 63 != 0;
        let last_idx = i - bit + (Self::BITS_PER_CELL - 1);
        let cell_idx = i / Self::BITS_PER_CELL;
        // We don't want to add new cells unless true, but we do want to shift
        // all cells over.  Therefore, if the next cell exists, then we should
        // shift into it.
        if last_bit_in_cell || cell_idx + 2 <= self.0.len() {
            self.insert(last_idx + 1, last_bit_in_cell);
        }
        let right_mask = 2u64.pow(bit as u32) - 1;
        let right = self.get_cell(i) & right_mask;
        let left = self.get_cell(i) & !right_mask;
        {
            let cell = self.get_cell_mut(i);
            *cell = (left << 1) | right;
        }
    }
}

impl fmt::Debug for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, bits) in self.0.iter().enumerate() {
            for i in (0..8).into_iter().rev() {
                write!(f, "{:08b}", ((bits & ((2u64.pow(8) - 1) << i*8)) >> i*8) as u8)?;
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
