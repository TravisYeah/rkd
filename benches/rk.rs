use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let data: [u8; 3] = [0, 1, 0];
  let q = 10_usize.pow(9) + 9;
  let rk = rkpb::RabinKarp::new(q);
  c.bench_function("search", |b| b.iter(|| rk.search(&[0, 0], &data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
