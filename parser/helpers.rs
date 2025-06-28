pub fn get_header(profile_data_lines: &[&str]) -> (Vec<String>, usize) {
    let profile_type_line = profile_data_lines.get(1).expect("No second element found");
    let profile_type = profile_type_line.strip_prefix("Type: ").expect("Invalid profile type format");

    let header_size = if profile_type == "cpu" { 6 } else { 5 };
    let header = profile_data_lines
        .get(0..header_size)
        .expect("No header found")
        .iter()
        .map(|s| s.to_string())
        .collect();

    return (header, header_size);
}

pub fn get_header_basic_fields(header: &[String]) -> (String, String, String) {
    let file_name_line = header.get(0).expect("No file name found");
    let file_name = file_name_line.strip_prefix("File: ").expect("Invalid file line format").trim();

    let profile_type_line = header.get(1).expect("No profile type found");
    let profile_type = profile_type_line.strip_prefix("Type: ").expect("Invalid profile type format").trim();

    let profile_timespamp_line = header.get(2).expect("No time stamp found");
    let profile_timespamp = profile_timespamp_line.strip_prefix("Time: ").expect("Invalid time stamp format").trim();

    return (file_name.to_string(), profile_type.to_string(), profile_timespamp.to_string());
}

pub fn get_header_parallelism_info(header: &[String]) -> (String, String, String) {
    let duration_samples_line = header.get(3).expect("No duration found");
    let duration = duration_samples_line
        .strip_prefix("Duration: ")
        .expect("Invalid duration format")
        .split(',')
        .next()
        .expect("Missing duration value")
        .trim();

    let total_samples_line = duration_samples_line.split('=').nth(1).expect("No '=' found").trim();

    let total_samples_time = total_samples_line.split('(').next().expect("Missing opening parenthesis").trim();

    let total_samples_percentage = total_samples_line
        .split('(')
        .nth(1)
        .expect("No opening parenthesis found")
        .trim_end_matches(')')
        .trim_end_matches('%')
        .trim();

    return (duration.to_string(), total_samples_time.to_string(), total_samples_percentage.to_string());
}

pub fn get_header_total_nodes_info(header: &[String]) -> (String, String, String) {
    let total_nodes_line = header.get(4).expect("No total nodes found");
    let collected_nodes_accounting_time_line = total_nodes_line
        .strip_prefix("Showing nodes accounting for ")
        .expect("Invalid total nodes format");

    let collected_nodes_accounting_time = collected_nodes_accounting_time_line.split(',').next().expect("Missing comma").trim();

    let collected_nodes_accounting_percentage = collected_nodes_accounting_time_line
        .split(',')
        .nth(1)
        .expect("Missing percentage part")
        .split('%')
        .next()
        .expect("Missing % sign")
        .trim();

    let total_nodes_accounting_time = collected_nodes_accounting_time_line.split("of").nth(1).expect("Missing 'of' part").trim();

    return (
        collected_nodes_accounting_time.to_string(),
        collected_nodes_accounting_percentage.to_string(),
        total_nodes_accounting_time.to_string(),
    );
}
