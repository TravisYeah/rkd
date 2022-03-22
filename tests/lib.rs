#[cfg(test)]
mod tests {
  #[test]
  fn search() {
    let data = Vec::from([1, 2, 1, 3]);
    let q = 10_usize.pow(9) + 9;
    let rk = rkpb::RabinKarp::new(q);
    let mut indices1 = vec![-1; 1];
    let mut indices2 = vec![-1; 1];
    let mut indices3 = vec![-1; 1];
    let mut indices4 = vec![-1; 2];
    let v1 = Vec::from([1, 1]);
    let v2 = Vec::from([1, 2]);
    let v3 = Vec::from([2, 1]);
    let v4 = Vec::from([1, 1, 3]);
    rk.search(&v1, &data, 2, &mut indices1);
    rk.search(&v2, &data, 2, &mut indices2);
    rk.search(&v3, &data, 2, &mut indices3);
    rk.search(&v4, &data, 2, &mut indices4);
    assert_eq!(indices1, [-1]);
    assert_eq!(indices2, [0]);
    assert_eq!(indices3, [1]);
    assert_eq!(indices4, [-1, 2]);
  }

  #[test]
  fn search_text() {
    use std::io::Read;
    let mut source_file = std::fs::File::open("benches/data/a.txt").unwrap();
    let mut target_file = std::fs::File::open("benches/data/b.txt").unwrap();
    let mut source = Vec::new();
    let mut target = Vec::new();
    source_file.read_to_end(&mut source).unwrap();
    target_file.read_to_end(&mut target).unwrap();
    let size = 2;
    let q = 10_usize.pow(9) + 9;
    let rk = rkpb::RabinKarp::new(q);
    let mut indices = vec![-1; source.len() - size + 1];
    rk.search(&source, &target, size, &mut indices);
    assert_eq!(indices, [0, 1, 2, -1, 6, 7, 8]);
  }

  #[test]
  fn search_greedy() {
    let data = Vec::from([1, 2, 1, 3]);
    let q = 10_usize.pow(9) + 9;
    let rk = rkpb::RabinKarp::new(q);
    let mut indices1: Vec<rkpb::Match> = Vec::new();
    let mut indices2: Vec<rkpb::Match> = Vec::new();
    let mut indices3: Vec<rkpb::Match> = Vec::new();
    let mut indices4: Vec<rkpb::Match> = Vec::new();
    let v1 = Vec::from([1, 1]);
    let v2 = Vec::from([1, 2]);
    let v3 = Vec::from([2, 1]);
    let v4 = Vec::from([1, 1, 3]);
    rk.search_greedy(&v1, &data, 2, &mut indices1);
    rk.search_greedy(&v2, &data, 2, &mut indices2);
    rk.search_greedy(&v3, &data, 2, &mut indices3);
    rk.search_greedy(&v4, &data, 2, &mut indices4);
    assert_eq!(indices1, []);
    assert_eq!(
      indices2,
      [rkpb::Match {
        source: 0,
        target: 0,
        size: 2
      }]
    );
    assert_eq!(
      indices3,
      [rkpb::Match {
        source: 0,
        target: 1,
        size: 2
      }]
    );
    assert_eq!(
      indices4,
      [rkpb::Match {
        source: 1,
        target: 2,
        size: 2
      }]
    );
  }

  #[test]
  fn search_greedy_extended_window() {
    let data = Vec::from([1, 2, 1, 3, 4, 5]);
    let q = 10_usize.pow(9) + 9;
    let rk = rkpb::RabinKarp::new(q);
    let mut indices: Vec<rkpb::Match> = Vec::new();
    let v = Vec::from([2, 1, 3, 4]);
    rk.search_greedy(&v, &data, 2, &mut indices);
    assert_eq!(
      indices,
      [rkpb::Match {
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
    let rk = rkpb::RabinKarp::new(q);
    let mut copies: Vec<rkpb::Match> = Vec::new();
    let vs = Vec::from([1, 1, 3]);
    let window = 2;
    rk.search_greedy(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    assert_eq!(
      delta,
      [
        rkpb::ADD,
        0,
        0,
        0,
        2,
        1,
        2,
        rkpb::COPY,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        2,
        rkpb::ADD,
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
    let rk = rkpb::RabinKarp::new(q);
    let mut copies: Vec<rkpb::Match> = Vec::new();
    let vs = Vec::from([1, 2, 3, 1]);
    let window = 2;
    rk.search_greedy(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    assert_eq!(
      delta,
      [
        rkpb::COPY,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        2,
        rkpb::ADD,
        0,
        0,
        0,
        1,
        1,
        rkpb::COPY,
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
    let rk = rkpb::RabinKarp::new(q);
    let mut copies: Vec<rkpb::Match> = Vec::new();
    let vs = Vec::from([1, 2, 9, 6, 3, 1]);
    let window = 2;
    rk.search_greedy(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    assert_eq!(
      delta,
      [
        rkpb::COPY,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        3,
        rkpb::ADD,
        0,
        0,
        0,
        2,
        1,
        5,
        rkpb::COPY,
        0,
        0,
        0,
        3,
        0,
        0,
        0,
        3,
        rkpb::ADD,
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
    let rk = rkpb::RabinKarp::new(q);
    let mut copies: Vec<rkpb::Match> = Vec::new();
    let vs = Vec::from([1, 2, 9, 8, 7, 2, 6, 3, 1, 2, 4, 3]);
    let window = 3;
    rk.search_greedy(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    assert_eq!(
      delta,
      [
        rkpb::COPY,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        6,
        rkpb::ADD,
        0,
        0,
        0,
        3,
        1,
        4,
        5,
        rkpb::COPY,
        0,
        0,
        0,
        6,
        0,
        0,
        0,
        6,
        rkpb::ADD,
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
    let rk = rkpb::RabinKarp::new(q);
    let mut copies: Vec<rkpb::Match> = Vec::new();
    let vs = Vec::from([1, 1, 3]);
    let window = 2;
    rk.search_greedy(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    let mut decompressed_data = Vec::new();
    rk.decompress(&vs, &mut decompressed_data, &delta);
    assert_eq!(decompressed_data, data);
  }

  #[test]
  fn decompress_2() {
    let data = Vec::from([1, 2, 1, 3, 1]);
    let q = 10_usize.pow(9) + 9;
    let rk = rkpb::RabinKarp::new(q);
    let mut copies: Vec<rkpb::Match> = Vec::new();
    let vs = Vec::from([1, 2, 3, 1]);
    let window = 2;
    rk.search_greedy(&vs, &data, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&data, &mut copies, &mut delta);
    let mut decompressed_data = Vec::new();
    rk.decompress(&vs, &mut decompressed_data, &delta);
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
    let rk = rkpb::RabinKarp::new(q);
    let mut copies: Vec<rkpb::Match> = Vec::new();
    let window = 4;
    rk.search_greedy(&source, &target, window, &mut copies);
    let mut delta = Vec::new();
    rk.compress(&target, &copies, &mut delta);

    let mut decompressed_data = Vec::new();
    rk.decompress(&source, &mut decompressed_data, &delta);
    assert_eq!(decompressed_data, target);
  }
}
