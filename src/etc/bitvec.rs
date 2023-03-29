const USIZE_BITS: usize = usize::BITS as usize;

const TRUE: bool = true;
const FALSE: bool = false;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct BitVec {
    buf: Vec<usize>,
    len: usize,
}

impl BitVec {
    pub fn new() -> Self {
        BitVec {
            buf: Vec::new(),
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        BitVec {
            buf: Vec::with_capacity(capacity / USIZE_BITS),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn set(&mut self, index: usize, value: bool) {
        // panic if out of bounds
        if index >= self.len {
            panic!("Index out of bounds");
        }
        let word = index / USIZE_BITS;
        let bit = index % USIZE_BITS;
        let mask = 1 << bit;
        if value {
            self.buf[word] |= mask;
        } else {
            self.buf[word] &= !mask;
        }
    }

    /// Push a value onto the end of the bit vector. Expapand self.buf if necessary.
    pub fn push(&mut self, value: bool) {
        if self.len % USIZE_BITS == 0 {
            self.buf.push(0);
        }
        self.len += 1;
        if value {
            self.set(self.len - 1, value);
        }
    }

    /// Pad Self to a specific length. Does nothing if already at or above length.
    /// Fills new bits with false.
    pub fn pad_to_len(&mut self, length: usize) {
        while self.buf.len() / USIZE_BITS < length {
            self.buf.push(0);
        }
        self.len = length;
    }
}

impl std::ops::Index<usize> for BitVec {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        // panic if out of bounds
        if index >= self.len {
            panic!("Index out of bounds");
        }
        let word = index / USIZE_BITS;
        let bit = index % USIZE_BITS;
        let mask = 1 << bit;
        if self.buf[word] & mask == 0 {
            &FALSE
        } else {
            &TRUE
        }
    }
}
