#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let data: [u8; 3] = [0, 1, 0];
    let q = 10_usize.pow(9) + 9;
    assert_eq!(rkpb::search(&[0, 0], &data, q), -1);
    assert_eq!(rkpb::search(&[0, 1], &data, q), 0);
    assert_eq!(rkpb::search(&[1, 0], &data, q), 1);
  }
}
