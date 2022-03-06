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
}
