//! BitMask: compact state using a u128 mask with bit helpers.

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct BitMask { bits: u128 }

impl BitMask {
    pub fn new() -> Self { Self { bits: 0 } }
    pub fn with_bits(bits: u128) -> Self { Self { bits } }
    pub fn set(&mut self, i: u32) { self.bits |= 1u128 << i; }
    pub fn clear(&mut self, i: u32) { self.bits &= !(1u128 << i); }
    pub fn toggle(&mut self, i: u32) { self.bits ^= 1u128 << i; }
    pub fn test(&self, i: u32) -> bool { (self.bits >> i) & 1 == 1 }
    pub fn count_ones(&self) -> u32 { self.bits.count_ones() }
    pub fn value(&self) -> u128 { self.bits }
}

impl From<u128> for BitMask { fn from(v: u128) -> Self { Self::with_bits(v) } }

#[cfg(test)]
mod tests {
    use super::BitMask;
    #[test]
    fn basic() {
        let mut b = BitMask::new();
        b.set(0); b.set(3);
        assert!(b.test(0) && b.test(3));
        b.toggle(3);
        assert!(!b.test(3));
        b.clear(0);
        assert_eq!(b.count_ones(), 0);
        assert_eq!(b.value(), 0);
    }

    #[test]
    fn from_and_with_bits() {
        let b = BitMask::with_bits(0b1010);
        assert!(b.test(1) && b.test(3));
        let c: BitMask = 0b11u128.into();
        assert!(c.test(0) && c.test(1));
    }
}
