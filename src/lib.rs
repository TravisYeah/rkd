pub static ADD: u8 = 0;
pub static COPY: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Match {
  pub source: usize,
  pub target: usize,
  pub size: usize,
}

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
    indices: &mut Vec<Match>,
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
      let mut j = last_j;
      while j < target_hashes.len() {
        if source_hashes[i] == target_hashes[j]
          && source[i..(i + window)] == target[j..(j + window)]
        {
          indices.push(Match {
            source: i,
            target: j,
            size: window,
          });
          last_j += window;
          i += window - 1;
          break;
        }
        j += 1;
      }
      i += 1;
    }
  }

  fn add(&self, target: &Vec<u8>, start: usize, end: usize, delta: &mut Vec<u8>) {
    delta.append(&mut Vec::from([ADD]));
    let add_size: u32 = (end - start).try_into().unwrap();
    delta.append(&mut Vec::from(add_size.to_be_bytes()));
    let mut add_data = Vec::new();
    add_data.resize(end - start, 0);
    add_data.clone_from_slice(&target[start..end]);
    delta.append(&mut add_data);
  }

  pub fn compress(&self, target: &Vec<u8>, copies: &mut Vec<Match>, delta: &mut Vec<u8>) {
    if copies.len() == 0 {
      return;
    }
    let mut last_copy_ix: usize = 0;
    for &m in copies.iter() {
      if m.target > last_copy_ix {
        self.add(target, last_copy_ix, m.target, delta);
      }
      delta.append(&mut Vec::from([COPY]));
      let copy_offset: u32 = m.source.try_into().unwrap();
      delta.append(&mut Vec::from(copy_offset.to_be_bytes()));
      let copy_size: u32 = m.size.try_into().unwrap();
      delta.append(&mut Vec::from(copy_size.to_be_bytes()));
      last_copy_ix = m.target + m.size;
    }
    let last_ix = copies[copies.len() - 1];
    let end_of_last_ix = last_ix.target + last_ix.size;
    if target.len() > end_of_last_ix {
      self.add(target, end_of_last_ix, target.len(), delta);
    }
  }
  fn read_u32(&self, bytes: &Vec<u8>, offset: usize) -> u32 {
    u32::from_be_bytes(bytes[offset..(offset + 4)].try_into().unwrap())
  }
  pub fn decompress(&self, source: &Vec<u8>, target: &mut Vec<u8>, delta: &Vec<u8>) {
    let mut i = 0;
    while i < delta.len() {
      let action = delta[i];
      i += 1;
      if action == ADD {
        let size = self.read_u32(delta, i) as usize;
        i += 4;
        target.extend(delta[i..(i + size)].iter());
        i += size;
      }
      if action == COPY {
        let offset = self.read_u32(delta, i) as usize;
        i += 4;
        let size = self.read_u32(delta, i) as usize;
        i += 4;
        target.extend(source[offset..(offset + size)].iter());
      }
    }
  }
}
