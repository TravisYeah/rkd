// Rabin-Karp search

pub fn search(pattern: &[u8], text: &[u8], q: usize) -> isize {
  let m = pattern.len();
  let n = text.len();
  let mut p = 0;
  let mut t = 0;
  let mut h = 1;
  let d = 2 ^ 8;

  // Compute horner's constant
  for _ in 0..(m - 1) {
    h = (h * d) % q;
  }

  // Calculate hash value for pattern and text
  for i in 0..m {
    p = (d * p + pattern[i] as usize) % q;
    t = (d * t + text[i] as usize) % q;
  }

  // Find the match
  for i in 0..(n - m + 1) {
    if p == t {
      let mut all = true;
      for j in 0..m {
        if text[i + j] != pattern[j] {
          all = false;
          break;
        }
      }
      if all {
        return i as isize;
      }
    }

    if i < n - m {
      t = (d * (t - text[i] as usize * h) + text[i + m] as usize) % q;

      // t can potentially be negative so we add q if needed
      if t < 0 {
        t = t + q;
      }
    }
  }

  return -1;
}
