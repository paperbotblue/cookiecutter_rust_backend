use std::fs::OpenOptions;
use std::io::Write;

pub fn append_to_file(file_name: &str, content: &str) {
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(file_name)
        .unwrap_or_else(|_| panic!("Unable to open file {}", file_name));

    data_file
        .write_all(content.as_bytes())
        .expect("write failed");
}
