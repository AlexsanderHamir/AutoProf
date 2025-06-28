mod parser;

fn main() {
    let profile_file_path = "tag_name_example/text/BenchmarkGenPool/BenchmarkGenPool_cpu.txt";
    parser::profile_parsing::extract_profile_data(profile_file_path);
}
