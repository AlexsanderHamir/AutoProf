use criterion::{Criterion, criterion_group, criterion_main};
use gocortex::parser::interface::parse_profile_data;
use std::path::PathBuf;

pub fn parser_benchmark(c: &mut Criterion) {
    let profile_file_path = PathBuf::from("tag_name_example/text/BenchmarkGenPool/BenchmarkGenPool_cpu.txt");
    c.bench_function("parse_profile_data", |b| {
        b.iter(|| match parse_profile_data(&profile_file_path) {
            Ok((_header, _functions_profile_data)) => {}
            Err(e) => {
                eprintln!("Error parsing profile data: {}", e);
                std::process::exit(1);
            }
        })
    });
}

criterion_group!(benches, parser_benchmark);
criterion_main!(benches);
