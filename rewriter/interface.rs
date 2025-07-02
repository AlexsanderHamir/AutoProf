use crate::parser::types::FunctionProfileData;

pub fn rewrite_profile_data(header_string: String, functions_profile_data: Vec<FunctionProfileData>) -> String {
    let mut rewritten_profile_data = String::new();

    rewritten_profile_data.push_str(&header_string);

    rewritten_profile_data.push_str(&format!(
        "{:<8} {:<8} {:<10} {:<8} {:<10} {}\n",
        "Flat", "Flat%", "Sum%", "Cum", "Cum%", "Function"
    ));

    for entry in functions_profile_data {
        rewritten_profile_data.push_str(&format!(
            "{:<8.2} {:<8} {:<10.2} {:<8.2} {:<10.2} {}\n",
            entry.flat, entry.flat_percentage, entry.sum_percentage, entry.cum, entry.cum_percentage, entry.function_name
        ));
    }

    rewritten_profile_data
}
