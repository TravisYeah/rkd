use std::io::Read;

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let mut source_file = std::fs::File::open("benches/data/rk-wiki.txt").unwrap();
  let mut target_file = std::fs::File::open("benches/data/rk-wiki-insert-p.txt").unwrap();
  let mut source = Vec::new();
  let mut target = Vec::new();
  source_file.read_to_end(&mut source).unwrap();
  target_file.read_to_end(&mut target).unwrap();
  println!("");
  println!("################ Test Information ################");
  println!("File size: {}kb", source.len() as isize / (1 << 10));
  println!("##################################################");
  println!("");
  let window = 1 << 8;
  let q = 10_usize.pow(9) + 9;

  let mut group = c.benchmark_group("search");
  group.sample_size(20);
  group.bench_function("search", |b| {
    b.iter(|| {
      let rk = rk_delta::RabinKarpDelta::new(q);
      let mut indices: Vec<rk_delta::Match> = Vec::new();
      rk.search(&source, &target, window, &mut indices);
    })
  });
  group.bench_function("compress", |b| {
    let rk = rk_delta::RabinKarpDelta::new(q);
    let mut indices: Vec<rk_delta::Match> = Vec::new();
    rk.search(&source, &target, window, &mut indices);
    b.iter(|| {
      let mut delta = Vec::new();
      rk.compress(&target, &indices, &mut delta);
    })
  });
  group.bench_function("decompress", |b| {
    let rk = rk_delta::RabinKarpDelta::new(q);
    let mut indices: Vec<rk_delta::Match> = Vec::new();
    rk.search(&source, &target, window, &mut indices);
    let mut delta = Vec::new();
    rk.compress(&target, &indices, &mut delta);
    b.iter(|| {
      let mut decompressed_data = Vec::new();
      rk.decompress(&source, &mut decompressed_data, &delta);
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
