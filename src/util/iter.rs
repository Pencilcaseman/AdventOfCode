pub struct HammingBitIter {
    limit: u64,
    k: u32,
    current: u64,
}

impl HammingBitIter {
    pub fn new(limit: u64) -> Self {
        Self { limit, k: 1, current: 1 }
    }
}

impl Iterator for HammingBitIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.limit {
            self.k += 1;
            self.current = (1u64 << self.k).wrapping_sub(1);

            if self.current >= self.limit {
                return None;
            }
        }

        let result = self.current;
        let x = self.current;
        let c = x & 0u64.wrapping_sub(x);
        let r = x.wrapping_add(c);

        let shift_amt = c.trailing_zeros();

        self.current = (((r ^ x) >> 2) >> shift_amt) | r;

        Some(result)
    }
}
