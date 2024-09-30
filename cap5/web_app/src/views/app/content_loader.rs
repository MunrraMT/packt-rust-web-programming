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

pub fn add_component(component_tag: String, html_data: String) -> String {
    let css_tag = format!("/*{}_CSS*/", component_tag.to_uppercase());
    let css_path =
        String::from("src/templates/components/") + &component_tag.to_lowercase() + ".css";
    let css_loader = read_file(&css_path);

    let html_tag = format!("<!--{}_HTML-->", component_tag.to_uppercase());
    let html_path =
        String::from("src/templates/components/") + &component_tag.to_lowercase() + ".html";
    let html_loader = read_file(&html_path);

    return html_data
        .replace(html_tag.as_str(), &html_loader)
        .replace(css_tag.as_str(), &css_loader);
}
