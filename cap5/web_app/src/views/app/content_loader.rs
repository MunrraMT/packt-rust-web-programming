pub fn read_file(file_path: &str) -> String {
    let data = std::fs::read_to_string(file_path).unwrap_or_else(|error| {
        dbg!(error);
        let html_page_not_found = read_file_page_not_found();
        return html_page_not_found;
    });

    return data;
}

pub fn read_file_page_not_found() -> String {
    let data = std::fs::read_to_string("src/templates/404.html").unwrap_or_else(|error| {
        dbg!(error);
        return "Page not found".to_string();
    });

    return data;
}
