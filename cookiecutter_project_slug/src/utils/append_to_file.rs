use std::fs::OpenOptions;
use std::io::Write;

pub fn append_to_file(file_name: &str, content: &str) {
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(file_name)
        .expect(format!("Unable to open file {}", file_name).as_str());

    data_file
        .write_all(content.as_bytes())
        .expect("write failed");
}
