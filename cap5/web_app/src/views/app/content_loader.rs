pub fn read_file(file_path: &str) -> String {
    let data = std::fs::read_to_string(file_path).unwrap_or("".to_string());
    return data;
}
