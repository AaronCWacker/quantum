/// Represents a non-quantum register of _width()_ bits.
///
/// We store this inefficiently for clarity.
pub struct ClassicalRegister {
    bits: Vec<u8>
}

impl ClassicalRegister {

    /// Construct a new non-quantum register, given a vector of ones and zeroes.
    ///
    /// The width is automatically determined from the vector.
    pub fn new(bits: Vec<u8>) -> ClassicalRegister {
        for bit in &bits {
            assert!(0 == *bit || 1 == *bit);
        }

        ClassicalRegister {
            bits: bits
        }
    }

    /// Construct zeroe-initialized non-quantum register of given width.
    pub fn zeroed(width: usize) -> ClassicalRegister {
        ClassicalRegister::new(vec![0; width])
    }

    /// Compute the register's width.
    pub fn width(&self) -> usize {
        self.bits.len()
    }

    /// Compute the current _state_ of the register.
    ///
    /// The _state_ is an integer which uniquely specifies all register bits (for a
    /// given width).  It does this in the obvious way, by enumerating all _2^n_ bit
    /// strings in the reversed lexicographic order, and assigning each string an index.
    ///
    /// This is equivalent to interpreting the register as an integer with the leftmost
    /// bit of least significance.
    ///
    /// # Panics
    ///
    /// This only works for registers of width <= 32.
    ///
    /// # Examples
    ///
    /// ```
    /// use quantum::classical::ClassicalRegister;
    ///
    /// assert_eq!(0, ClassicalRegister::new(vec![0, 0, 0]).state());
    /// assert_eq!(6, ClassicalRegister::new(vec![0, 1, 1]).state());
    /// ```
    pub fn state(&self) -> u32 {
        let mut state = 0u32;

        for (pos, bit) in self.bits.iter().enumerate() {
            if 0u8 != *bit {
                state += 2u32.pow(pos as u32);
            }
        }

        state
    }
}
