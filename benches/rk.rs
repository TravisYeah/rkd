use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let data: [u8; 3] = [0, 1, 0];
  let q = 10_usize.pow(9) + 9;
  c.bench_function("search", |b| b.iter(|| rkpb::search(&[0, 0], &data, q)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
