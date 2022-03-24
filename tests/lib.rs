#[cfg(test)]
mod tests {
  #[test]
  fn search() {
    let data = Vec::from([1, 2, 1, 3]);
    let q = 10_usize.pow(9) + 9;
    let rk = rk_delta::RabinKarpDelta::new(q);
    let mut indices1: Vec<rk_delta::Match> = Vec::new();
    let mut indices2: Vec<rk_delta::Match> = Vec::new();
    let mut indices3: Vec<rk_delta::Match> = Vec::new();
    let mut indices4: Vec<rk_delta::Match> = Vec::new();
    let v1 = Vec::from([1, 1]);
    let v2 = Vec::from([1, 2]);
    let v3 = Vec::from([2, 1]);
    let v4 = Vec::from([1, 1, 3]);
    rk.search(&v1, &data, 2, &mut indices1);
    rk.search(&v2, &data, 2, &mut indices2);
    rk.search(&v3, &data, 2, &mut indices3);
    rk.search(&v4, &data, 2, &mut indices4);
    assert_eq!(indices1, []);
    assert_eq!(
      indices2,
      [rk_delta::Match {
        source: 0,
        target: 0,
        size: 2
      }]
    );
    assert_eq!(
      indices3,
      [rk_delta::Match {
        source: 0,
        target: 1,
        size: 2
      }]
    );
    assert_eq!(
      indices4,
      [rk_delta::Match {
        source: 1,
        target: 2,
        size: 2
      }]
    );
  }

  #[test]
  fn search_extended_window() {
    let data = Vec::from([1, 2, 1, 3, 4, 5]);
    let q = 10_usize.pow(9) + 9;
    let rk = rk_delta::RabinKarpDelta::new(q);
    let mut indices: Vec<rk_delta::Match> = Vec::new();
    let v = Vec::from([2, 1, 3, 4]);
    rk.search(&v, &data, 2, &mut indices);
    assert_eq!(
      indices,
      [rk_delta::Match {
        source: 0,
        target: 1,
        size: 4
      }]
    );
  }

  #[test]
  fn compress() {
    let data = Vec::from([1, 2, 1, 3, 1]);
    let q = 10_usize.pow(9) + 9;
    let rk = rk_delta::RabinKarpDelta::new(q);
    let mut copies: Vec<rk_delta::Match> = Vec::new();
    let vs = Vec::from([1, 1, 3]);
    let window = 2;
    rk.search(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    assert_eq!(
      delta,
      [
        rk_delta::ADD,
        0,
        0,
        0,
        2,
        1,
        2,
        rk_delta::COPY,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        2,
        rk_delta::ADD,
        0,
        0,
        0,
        1,
        1,
      ]
    );
  }

  #[test]
  fn compress_test_2() {
    let data = Vec::from([1, 2, 1, 3, 1]);
    let q = 10_usize.pow(9) + 9;
    let rk = rk_delta::RabinKarpDelta::new(q);
    let mut copies: Vec<rk_delta::Match> = Vec::new();
    let vs = Vec::from([1, 2, 3, 1]);
    let window = 2;
    rk.search(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    assert_eq!(
      delta,
      [
        rk_delta::COPY,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        2,
        rk_delta::ADD,
        0,
        0,
        0,
        1,
        1,
        rk_delta::COPY,
        0,
        0,
        0,
        2,
        0,
        0,
        0,
        2
      ]
    );
  }

  #[test]
  fn compress_test_3() {
    let data = Vec::from([1, 2, 9, 1, 5, 6, 3, 1, 4]);
    let q = 10_usize.pow(9) + 9;
    let rk = rk_delta::RabinKarpDelta::new(q);
    let mut copies: Vec<rk_delta::Match> = Vec::new();
    let vs = Vec::from([1, 2, 9, 6, 3, 1]);
    let window = 2;
    rk.search(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    assert_eq!(
      delta,
      [
        rk_delta::COPY,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        3,
        rk_delta::ADD,
        0,
        0,
        0,
        2,
        1,
        5,
        rk_delta::COPY,
        0,
        0,
        0,
        3,
        0,
        0,
        0,
        3,
        rk_delta::ADD,
        0,
        0,
        0,
        1,
        4,
      ]
    );
  }

  #[test]
  fn compress_test_4() {
    let data = Vec::from([1, 2, 9, 8, 7, 2, 1, 4, 5, 6, 3, 1, 2, 4, 3, 5]);
    let q = 10_usize.pow(9) + 9;
    let rk = rk_delta::RabinKarpDelta::new(q);
    let mut copies: Vec<rk_delta::Match> = Vec::new();
    let vs = Vec::from([1, 2, 9, 8, 7, 2, 6, 3, 1, 2, 4, 3]);
    let window = 3;
    rk.search(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    assert_eq!(
      delta,
      [
        rk_delta::COPY,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        6,
        rk_delta::ADD,
        0,
        0,
        0,
        3,
        1,
        4,
        5,
        rk_delta::COPY,
        0,
        0,
        0,
        6,
        0,
        0,
        0,
        6,
        rk_delta::ADD,
        0,
        0,
        0,
        1,
        5
      ]
    );
  }

  #[test]
  fn decompress() {
    let data = Vec::from([1, 2, 1, 3, 1]);
    let q = 10_usize.pow(9) + 9;
    let rk = rk_delta::RabinKarpDelta::new(q);
    let mut copies: Vec<rk_delta::Match> = Vec::new();
    let vs = Vec::from([1, 1, 3]);
    let window = 2;
    rk.search(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    let mut decompressed_data = Vec::new();
    rk_delta::RabinKarpDelta::decompress(&vs, &mut decompressed_data, &delta);
    assert_eq!(decompressed_data, data);
  }

  #[test]
  fn decompress_2() {
    let data = Vec::from([1, 2, 1, 3, 1]);
    let q = 10_usize.pow(9) + 9;
    let rk = rk_delta::RabinKarpDelta::new(q);
    let mut copies: Vec<rk_delta::Match> = Vec::new();
    let vs = Vec::from([1, 2, 3, 1]);
    let window = 2;
    rk.search(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    let mut decompressed_data = Vec::new();
    rk_delta::RabinKarpDelta::decompress(&vs, &mut decompressed_data, &delta);
    assert_eq!(decompressed_data, data);
  }

  #[test]
  fn decompress_big_file() {
    use std::io::Read;
    let mut source_file = std::fs::File::open("benches/data/rk-wiki.txt").unwrap();
    let mut target_file = std::fs::File::open("benches/data/rk-wiki-insert-p.txt").unwrap();
    let mut source = Vec::new();
    let mut target = Vec::new();
    source_file.read_to_end(&mut source).unwrap();
    target_file.read_to_end(&mut target).unwrap();

    let q = 10_usize.pow(9) + 9;
    let rk = rk_delta::RabinKarpDelta::new(q);
    let mut copies: Vec<rk_delta::Match> = Vec::new();
    let window = 4;
    rk.search(&source, &target, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&target, &copies, &mut delta);

    let mut decompressed_data = Vec::new();
    rk_delta::RabinKarpDelta::decompress(&source, &mut decompressed_data, &delta);
    assert_eq!(decompressed_data, target);
  }

  #[test]
  fn file_helper_methods() {
    use std::io::Read;
    let source = "benches/data/rk-wiki.txt";
    let target = "benches/data/rk-wiki-insert-p.txt";
    let delta = "benches/data/rk-wiki-insert-p-delta.txt";
    let target_recreated = "benches/data/rk-wiki-insert-p-recreated.txt";
    let mut target_file = std::fs::File::open(target).unwrap();
    let mut target_bytes = Vec::new();
    target_file.read_to_end(&mut target_bytes).unwrap();

    rk_delta::RabinKarpDelta::create_delta_file(&source, &target, &delta);
    let mut target_file_recreated = std::fs::File::open(target_recreated).unwrap();
    let mut target_bytes_recreated = Vec::new();
    rk_delta::RabinKarpDelta::create_target_file(source, target_recreated, delta);
    target_file_recreated
      .read_to_end(&mut target_bytes_recreated)
      .unwrap();
    assert_eq!(target_bytes, target_bytes_recreated);
  }

  #[test]
  fn testit() {
    use lzzzz::lz4;

    let data: Vec<u8> = Vec::from("The quick brown fox jumps over the lazy dog.");

    // LZ4 compression
    let mut comp = Vec::new();
    lz4::compress_to_vec(&data, &mut comp, lz4::ACC_LEVEL_DEFAULT).unwrap();

    // LZ4/LZ4_HC decompression
    let mut decomp = Vec::new();
    lz4::decompress(&comp, &mut decomp).unwrap();

    assert_eq!(data, decomp);
  }
}
