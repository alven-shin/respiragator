use std::{collections::VecDeque, time::Instant};

pub struct RingBuffer<const N: usize> {
    buffer: [f64; N],
    // index for next insertion
    idx: usize,
    sum: f64,
    is_breathing: bool,
    breaths: VecDeque<Instant>,
    is_filled: bool,
}

impl<const N: usize> RingBuffer<N> {
    pub fn new() -> Self {
        RingBuffer {
            buffer: [0.; N],
            idx: 0,
            sum: 0.,
            is_breathing: false,
            breaths: VecDeque::new(),
            is_filled: false,
        }
    }

    /// push to end of buffer
    pub fn push(&mut self, item: f64) {
        let old_value = self.buffer.get_mut(self.idx).unwrap();
        self.sum -= *old_value;

        *old_value = item;
        self.idx += 1;
        self.sum += item;

        if self.idx == self.buffer.len() {
            self.idx = 0;
            self.is_filled = true;
        }

        // increment breaths if flex sensor reading is above average
        // set is_breathing to true to prevent duplicate breaths
        if self.is_filled && !self.is_breathing && item > self.average() + 2. {
            self.is_breathing = true;
            self.breaths.push_back(Instant::now());
        // reset is_breathing once flex sensor reading returns to normal
        } else if self.is_breathing && item < self.average() + 1. {
            self.is_breathing = false;
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

    pub fn average(&self) -> f64 {
        self.sum / N as f64
    }

    pub fn breaths(&mut self) -> usize {
        let now = Instant::now();

        // remove old instants
        while let Some(&x) = self.breaths.front() {
            if (now - x).as_secs() > 10 {
                self.breaths.pop_front();
            } else {
                break;
            }
        }

        self.breaths.len()
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
