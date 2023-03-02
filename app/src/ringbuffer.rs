pub struct RingBuffer<const N: usize> {
    buffer: [f64; N],
    // index for next insertion
    idx: usize,
}

impl<const N: usize> RingBuffer<N> {
    pub fn new() -> Self {
        RingBuffer {
            buffer: [0.; N],
            idx: 0,
        }
    }

    /// push to end of buffer
    pub fn push(&mut self, item: f64) {
        *self.buffer.get_mut(self.idx).unwrap() = item;
        self.idx += 1;
        if self.idx == self.buffer.len() {
            self.idx = 0;
        }
    }

    /// get last inserted value
    pub fn last(&self) -> f64 {
        match self.idx.checked_sub(1).and_then(|x| self.buffer.get(x)) {
            Some(&x) => x,
            None => self.buffer.last().copied().unwrap(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = f64> + '_ {
        let first_half = &self.buffer.as_slice()[self.idx..];
        let sec_half = &self.buffer.as_slice()[..self.idx];
        first_half
            .into_iter()
            .copied()
            .chain(sec_half.into_iter().copied())
    }
}

impl<const N: usize> Extend<u8> for RingBuffer<N> {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        iter.into_iter().for_each(|x| self.push(x as f64));
    }
}

impl<const N: usize> AsRef<[f64]> for RingBuffer<N> {
    fn as_ref(&self) -> &[f64] {
        self.buffer.as_slice()
    }
}
