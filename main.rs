mod parser;

fn main() {
    let profile_file_path = "tag_name_example/text/BenchmarkGenPool/BenchmarkGenPool_cpu.txt";
    let (header, functions_profile_data) = parser::profile_parsing::extract_profile_data(profile_file_path);
    println!("{:#?}", header);
    println!("{:#?}", functions_profile_data);
}
