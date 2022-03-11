#[cfg(test)]
mod tests {
  #[test]
  fn search() {
    let data: [u8; 3] = [0, 1, 0];
    let q = 10_usize.pow(9) + 9;
    let rk = rkpb::RabinKarp::new(q);
    assert_eq!(rk.search(&[0, 0], &data), -1);
    assert_eq!(rk.search(&[0, 1], &data), 0);
    assert_eq!(rk.search(&[1, 0], &data), 1);
  }

  // #[test]
  // fn bloom() {
  //   let mut b = rkpb::Bloom::new();
  //   b.add(1);
  //   let mut v = vec![0; 1 << 9];
  //   v[1] = 1;
  //   assert_eq!(b.bloom, v);
  // }

  #[test]
  fn bloom_collision() {
    let mut b = rkpb::Bloom::new();
    let mut nums = std::collections::HashSet::new();
    let mut fp = 0.0;

    for _ in 0..(1 << 8) {
      let ix = rand::random::<usize>();
      nums.insert(ix);
      b.add(ix);
    }
    for _ in 0..(1 << 8) {
      let ix = rand::random::<usize>();
      if b.exists(ix) && nums.get(&ix) == None {
        fp += 1.0;
      }
    }
    let fpr = fp / (1 << 8) as f64;
    println!("{}", fpr);
    assert!(fpr < 0.001);
  }
}
