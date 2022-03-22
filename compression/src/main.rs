use csv::{Writer, WriterBuilder};
use std::io::Read;

fn fixed_width_write<'a>(writer: &mut Writer<std::fs::File>, record: &Vec<&'a str>) {
  writer
    .write_record(record.iter().map(|d| format!("{:<20}", d)))
    .unwrap();
}

fn delta_compress(source_path: &str, target_path: &str) {
  let mut writer = WriterBuilder::new()
    .delimiter(b'\t')
    .from_path("compression.csv")
    .unwrap();
  fixed_width_write(
    &mut writer,
    &Vec::from([
      "title",
      "description",
      "file_source",
      "file_target",
      "source_file_size",
      "target_file_size",
      "delta_size",
    ]),
  );
  let mut source_file = std::fs::File::open(source_path).unwrap();
  let mut target_file = std::fs::File::open(target_path).unwrap();
  let mut source = Vec::new();
  let mut target = Vec::new();
  source_file.read_to_end(&mut source).unwrap();
  target_file.read_to_end(&mut target).unwrap();
  let q = 10_usize.pow(9) + 9;
  let rk = rkpb::RabinKarp::new(q);
  let mut copies: Vec<rkpb::Match> = Vec::new();
  let window = 1 << 7;
  rk.search(&source, &target, window, &mut copies);
  let mut delta = Vec::new();
  rk.compress(&target, &copies, &mut delta);
  fixed_width_write(
    &mut writer,
    &Vec::from([
      "rabin karp wiki",
      "insert middle",
      "rk-wiki.txt",
      "rk-wiki-insert-p.txt",
      source.len().to_string().as_str(),
      target.len().to_string().as_str(),
      delta.len().to_string().as_str(),
    ]),
  );
}

fn main() {
  delta_compress(
    "../benches/data/rk-wiki.txt",
    "../benches/data/rk-wiki-insert-p.txt",
  );
}
