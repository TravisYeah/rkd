use std::io::Read;

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("search");
  group.sample_size(20);
  let mut source_file = std::fs::File::open("benches/data/rk-wiki.txt").unwrap();
  let mut target_file = std::fs::File::open("benches/data/rk-wiki-insert-p.txt").unwrap();
  let mut source = Vec::new();
  let mut target = Vec::new();
  source_file.read_to_end(&mut source).unwrap();
  target_file.read_to_end(&mut target).unwrap();
  println!("File size: {}kb", source.len() as isize / (1 << 10));
  let window = 1 << 8;
  let q = 10_usize.pow(9) + 9;
  group.bench_function("search", |b| {
    b.iter(|| {
      let rk = rkpb::RabinKarp::new(q);
      let mut indices = vec![-1; source.len() - window + 1];
      rk.search(&source, &target, window, &mut indices);
      rk.search(&source, &target, window, &mut indices);
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
