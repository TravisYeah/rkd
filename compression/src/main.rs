use csv::{Writer, WriterBuilder};
use std::io::Read;

fn fixed_width_write<'a>(writer: &mut Writer<std::fs::File>, record: &Vec<&'a str>) {
  writer
    .write_record(record.iter().map(|d| format!("{:<20}", d)))
    .unwrap();
}

fn delta_compress(writer: &mut Writer<std::fs::File>, title: &str, description: &str, source_path: &str, target_path: &str) {
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
  let source_parts = source_path.split("/").collect::<Vec<&str>>();
  let target_parts = target_path.split("/").collect::<Vec<&str>>();
  fixed_width_write(
    writer,
    &Vec::from([
      title, description,
      source_parts[source_parts.len() - 1],
      target_parts[target_parts.len() - 1],
      source.len().to_string().as_str(),
      target.len().to_string().as_str(),
      delta.len().to_string().as_str(),
    ]),
  );
}

fn main() {
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
  delta_compress(
    &mut writer,
    "rabin karp wiki",
    "insert middle",
    "../benches/data/rk-wiki.txt",
    "../benches/data/rk-wiki-insert-p.txt",
  );
  delta_compress(
    &mut writer,
    "rabin karp wiki",
    "multiple ops",
    "../benches/data/rk-wiki.txt",
    "../benches/data/rk-wiki-multi-ops.txt",
  );
}
