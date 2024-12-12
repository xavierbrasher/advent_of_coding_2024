use std::fs;

pub fn open_file(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect(format!("COULD NOT READ FILE. DOES {} EXIST?", filename).as_str())
}
