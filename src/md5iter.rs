use itoa;
use md5::{Context, Digest};

pub struct HashIter {
    idx: usize,
    hasher: Context,
}

impl HashIter {
    pub fn new(seed: &str) -> HashIter {
        let mut md5 = Context::new();
        md5.consume(seed.as_bytes());
        HashIter {
            idx: 0,
            hasher: md5,
        }
    }
}

impl Iterator for HashIter {
    type Item = Digest;

    #[inline(always)]
    fn next(&mut self) -> Option<Digest> {
        let mut hasher = self.hasher.clone();
        itoa::write(&mut hasher, self.idx).unwrap();
        self.idx += 1;
        Some(hasher.compute())
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use md5;
    use std::io::Write;

    #[bench]
    fn md5_an_md5(b: &mut test::Bencher) {
        let mut s = ::std::io::sink();
        b.iter(|| write!(s, "{:x}", md5::compute("577571be4de9dcce85a041ba0410f29f")));
    }
}
