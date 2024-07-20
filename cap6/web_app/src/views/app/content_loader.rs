use std::fs;

pub fn read_file(file_path: &str) -> String {
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    return data;
}

pub fn add_component(component_tag: String, html_data: String) -> String {
    let html_tag = "<!-- ".to_string() + &component_tag.to_uppercase() + "_HTML -->";
    let html_path =
        String::from("./templates/components/") + &component_tag.to_lowercase() + ".html";
    let html_loaded = read_file(&html_path);

    let css_tag = "/* ".to_string() + &component_tag.to_uppercase() + "_CSS */";
    let css_path = String::from("./templates/components/") + &component_tag.to_lowercase() + ".css";
    let css_loader = read_file(&css_path);

    let html_data = html_data
        .replace(&html_tag, &html_loaded)
        .replace(&css_tag, &css_loader);

    return html_data;
}
