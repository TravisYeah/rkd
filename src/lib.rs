use std::{io::Read, panic};

use lzzzz::lz4;

pub static ADD: u8 = 0;
pub static COPY: u8 = 1;
pub static RKD: &[u8] = "rkd".as_bytes();
pub static VERSION: [u8; 2] = [1, 0];
pub static MAX_VEC_SIZE: u32 = u32::MAX;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Match {
  pub source: usize,
  pub target: usize,
  pub size: usize,
}

pub struct RabinKarpDelta {
  d: usize,
  q: usize,
}

fn read_u32(bytes: &Vec<u8>, offset: usize) -> u32 {
  u32::from_be_bytes(bytes[offset..(offset + 4)].try_into().unwrap())
}

impl RabinKarpDelta {
  pub fn new(q: usize) -> RabinKarpDelta {
    RabinKarpDelta { d: 1 << 8, q }
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
    let mut h = 1;
    for _ in 0..(m - 1) {
      h = (h * self.d) % self.q;
    }
    return h;
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
    indices: &mut Vec<Match>,
  ) -> () {
    if source.len() > MAX_VEC_SIZE as usize {
      panic!("Source file larger than max ({}).", MAX_VEC_SIZE);
    }
    if target.len() > MAX_VEC_SIZE as usize {
      panic!("Target file larger than max ({}).", MAX_VEC_SIZE);
    }
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
      let mut ext = 0;
      let mut j = last_j;
      let mut found = false;
      while j < target_hashes.len() {
        if found {
          if source.len() > i + window + ext - 1
            && target.len() > j + window + ext - 1
            && source[i + window + ext - 1] == target[j + window + ext - 1]
          {
            let len = indices.len();
            let last_ix = indices[len - 1];
            indices[len - 1] = Match {
              source: last_ix.source,
              target: last_ix.target,
              size: last_ix.size + 1,
            };
            ext += 1;
            continue;
          }
        } else if source_hashes[i] == target_hashes[j]
          && source[i..(i + window)] == target[j..(j + window)]
        {
          found = true;
          indices.push(Match {
            source: i,
            target: j,
            size: window,
          });
          ext += 1;
          continue;
        }
        if found {
          let size = indices[indices.len() - 1].size;
          i += size - 1;
          last_j += size;
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

  pub fn compress(&self, target: &Vec<u8>, copies: &Vec<Match>, delta: &mut Vec<u8>) {
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
  pub fn decompress(source: &Vec<u8>, target: &mut Vec<u8>, delta: &Vec<u8>) {
    let mut i = 0;
    while i < delta.len() {
      let action = delta[i];
      i += 1;
      if action == ADD {
        let size = read_u32(delta, i) as usize;
        i += 4;
        target.extend(delta[i..(i + size)].iter());
        i += size;
      }
      if action == COPY {
        let offset = read_u32(delta, i) as usize;
        i += 4;
        let size = read_u32(delta, i) as usize;
        i += 4;
        target.extend(source[offset..(offset + size)].iter());
      }
    }
  }
  pub fn create_delta_file(source: &str, target: &str, delta: &str) {
    let mut source_file = std::fs::File::open(source).unwrap();
    let mut target_file = std::fs::File::open(target).unwrap();
    let mut source_bytes = Vec::new();
    let mut target_bytes = Vec::new();
    source_file.read_to_end(&mut source_bytes).unwrap();
    target_file.read_to_end(&mut target_bytes).unwrap();
    let q = 10_usize.pow(9) + 9;
    let rk = RabinKarpDelta::new(q);
    let mut copies: Vec<Match> = Vec::new();
    let window = 4;
    rk.search(&source_bytes, &target_bytes, window, &mut copies);
    let mut delta_bytes = Vec::new();
    rk.compress(&target_bytes, &copies, &mut delta_bytes);
    let mut compressed_delta_bytes = Vec::new();
    compressed_delta_bytes.append(&mut [RKD, &VERSION].concat());
    compressed_delta_bytes.append(&mut Vec::from((delta_bytes.len() as u32).to_be_bytes()));
    lz4::compress_to_vec(
      &delta_bytes,
      &mut compressed_delta_bytes,
      lz4::ACC_LEVEL_DEFAULT,
    )
    .unwrap();
    std::fs::write(delta, compressed_delta_bytes).unwrap();
  }
  pub fn create_target_file(source: &str, target: &str, delta: &str) {
    let mut source_file = std::fs::File::open(source).unwrap();
    let mut delta_file = std::fs::File::open(delta).unwrap();
    let mut source_bytes = Vec::new();
    source_file.read_to_end(&mut source_bytes).unwrap();
    let mut compressed_delta_bytes = Vec::new();
    delta_file.read_to_end(&mut compressed_delta_bytes).unwrap();
    if compressed_delta_bytes[0..3] != *RKD {
      panic!("Invalid RKD delta file.");
    }
    if compressed_delta_bytes[3] != VERSION[0] {
      panic!(
        "RKD version {}.{} cannot process delta file version {}.{}.",
        VERSION[0], VERSION[1], compressed_delta_bytes[3], compressed_delta_bytes[4]
      )
    }
    let decompressed_delta_size = read_u32(&compressed_delta_bytes, 5);
    let mut delta_bytes = vec![0; decompressed_delta_size.try_into().unwrap()];
    let delta_bytes_to_decompress = &compressed_delta_bytes[9..];
    lz4::decompress(&delta_bytes_to_decompress, &mut delta_bytes).unwrap();
    let mut decompressed_data = Vec::new();
    RabinKarpDelta::decompress(&source_bytes, &mut decompressed_data, &delta_bytes);
    std::fs::write(target, decompressed_data).unwrap();
  }
}
