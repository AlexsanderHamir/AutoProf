use once_cell::sync::Lazy;
use regex::Regex;

pub const CPU_HEADER_SIZE: usize = 6;
pub const REST_HEADER_SIZE: usize = 5;
pub const EMPTY_LINE_COUNT: usize = 1;

pub static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[\d.]+").unwrap());
