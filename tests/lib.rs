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
  fn compress() {
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
}
