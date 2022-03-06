pub struct RabinKarp {
  d: usize,
  q: usize,
}

impl RabinKarp {
  pub fn new(q: usize) -> RabinKarp {
    RabinKarp { d: 1 << 8, q }
  }
  pub fn hash(&self, items: &[u8], m: usize) -> usize {
    let mut res = 0;
    for i in 0..m {
      res = (self.d * res + items[i] as usize) % self.q;
    }
    return res;
  }
  pub fn roll(&self, hash: usize, prev: usize, next: usize, h: usize) -> usize {
    let mut res = hash;
    res = (self.d * (res - prev * h) + next) % self.q;
    // t can potentially be negative so we add q if needed
    if res < 0 {
      res = res + self.q;
    }
    res
  }
  pub fn horner_constant(&self, m: usize) -> usize {
    let mut h = 1;
    for _ in 0..(m - 1) {
      h = (h * self.d) % self.q;
    }
    h
  }
  // Rabin-Karp search
  pub fn search(&self, pattern: &[u8], text: &[u8]) -> isize {
    let m = pattern.len();
    let n = text.len();
    // Compute horner's constant
    let h = self.horner_constant(m);
    // Calculate hash value for pattern and text
    let p = self.hash(pattern, m);
    let mut t = self.hash(text, m);
    // Find the match
    for i in 0..(n - m + 1) {
      if p == t && text[i..(i + m)] == pattern[0..m] {
        return i as isize;
      }
      if i < n - m {
        t = self.roll(t, text[i] as usize, text[i + m] as usize, h);
      }
    }
    return -1;
  }
}
