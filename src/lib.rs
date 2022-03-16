static ADD: u8 = 0;
static COPY: u8 = 1;

pub struct RabinKarp {
  d: usize,
  q: usize,
}

impl RabinKarp {
  pub fn new(q: usize) -> RabinKarp {
    RabinKarp { d: 1 << 8, q }
  }
  fn hash(&self, items: &Vec<u8>, m: usize) -> usize {
    let mut res = 0;
    for i in 0..m {
      res = (self.d * res + items[i] as usize) % self.q;
    }
    return res;
  }
  fn roll(&self, hash: usize, prev: usize, next: usize, h: usize) -> usize {
    let mut res = hash;
    res = (res + self.q - h * prev % self.q) % self.q;
    res = (res * self.d + next) % self.q;
    res
  }
  fn horner_constant(&self, m: usize) -> usize {
    self.d.pow(m as u32 - 1) % self.q
  }
  fn precompute_hashes(
    &self,
    bytes: &Vec<u8>,
    window: usize,
    h: usize,
    hashes: &mut Vec<usize>,
  ) -> () {
    let mut hash = self.hash(bytes, window);
    hashes[0] = hash;
    if bytes.len() - window > 0 {
      for i in 0..(bytes.len() - window) {
        hash = self.roll(hash, bytes[i] as usize, bytes[i + window] as usize, h);
        hashes[i + 1] = hash;
      }
    }
  }
  pub fn search(
    &self,
    source: &Vec<u8>,
    target: &Vec<u8>,
    window: usize,
    indices: &mut Vec<isize>,
  ) -> () {
    let h = self.horner_constant(window);
    let mut source_hashes = Vec::new();
    let mut target_hashes = Vec::new();
    source_hashes.resize(source.len() - window + 1, 0);
    target_hashes.resize(target.len() - window + 1, 0);
    self.precompute_hashes(source, window, h, &mut source_hashes);
    self.precompute_hashes(target, window, h, &mut target_hashes);
    for (i, source_hash) in source_hashes.iter().enumerate() {
      for (j, target_hash) in target_hashes.iter().enumerate() {
        if source_hash == target_hash && source[i..(i + window)] == target[j..(j + window)] {
          indices[i] = j as isize;
          continue;
        }
      }
    }
  }
  pub fn search_greedy(
    &self,
    source: &Vec<u8>,
    target: &Vec<u8>,
    window: usize,
    indices: &mut Vec<isize>,
  ) -> () {
    let h = self.horner_constant(window);
    let mut source_hashes = Vec::new();
    let mut target_hashes = Vec::new();
    source_hashes.resize(source.len() - window + 1, 0);
    target_hashes.resize(target.len() - window + 1, 0);
    self.precompute_hashes(source, window, h, &mut source_hashes);
    self.precompute_hashes(target, window, h, &mut target_hashes);
    let mut i = 0;

    let mut last_j = 0;
    while i < source_hashes.len() {
      let source_hash = source_hashes[i];
      let mut j = last_j;
      while j < source_hashes.len() {
        if source_hash == target_hashes[j] && source[i..(i + window)] == target[j..(j + window)] {
          indices[i] = j as isize;
          j += window;
          last_j = j;
          i += window;
          break;
        }
        j += 1;
      }
      i += 1;
    }
  }

  // TODO handle ADDs size > 1 << 32
  // TODO handle COPY offsets and size > 1 << 32
  // TODO change to use tree structure in indices such that both the offset and the length are included (change would be in fn search_greedy)
  pub fn compress(
    &self,
    source: &Vec<u8>,
    indices: &mut Vec<isize>,
    delta: &mut Vec<u8>,
    window: usize,
  ) {
    let mut search_start: usize = 0;
    let mut i: usize = 0;
    while i < indices.len() {
      let ix = indices[i];
      if ix > 0 {
        if i > search_start {
          delta.append(&mut Vec::from([ADD]));
          let add_size: u32 = (i - search_start).try_into().unwrap();
          delta.append(&mut Vec::from(add_size.to_be_bytes()));
          let mut add_data = Vec::new();
          add_data.clone_from_slice(&source[search_start..i]);
          delta.append(&mut add_data);
        }
        delta.append(&mut Vec::from([COPY]));
        let copy_offset: u32 = (ix).try_into().unwrap();
        delta.append(&mut Vec::from(copy_offset.to_be_bytes()));
        let copy_size: u32 = window.try_into().unwrap();
        delta.append(&mut Vec::from(copy_size.to_be_bytes()));
        i += 1;
        search_start = i;
        continue;
      }
      i += 1;
    }
  }
}
