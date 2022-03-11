use bitvec::prelude::*;
pub struct Bloom {
  pub bloom: BitSlice,
}
impl Bloom {
  pub fn new() -> Bloom {
    Bloom {
      bloom: bits![u32, Lsb0; 0; 32],
    }
  }

  pub fn add(&self, ix: usize) -> () {
    let len = self.bloom.len();
    let index = ix % len;
    let mut bloom = *self.bloom.get_mut(ix % len).unwrap();
    bloom = true;
  }

  pub fn exists(&self, ix: usize) -> bool {
    let len = self.bloom.len();
    self.bloom[ix % len]
  }
}
