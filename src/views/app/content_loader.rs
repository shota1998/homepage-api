use std::fs;

/// Read and convert file to String. 
/// 
/// # Aruguments
/// * file_path (&str): Path to the file which contain to do items data.
/// 
/// # Reurns  
/// (Sring): Converted to do items data.
pub fn read_file(file_path: &str) -> String {
  let data: String = fs::read_to_string(file_path)
                         .expect("Unable to read file");
  return data
}

/// Replace {{TAG}} in html with loaded html/css.
/// Then, return the changed html.
/// 
/// # Arguments
/// * component_tag (String): Indicate where html/css will be injeced.
/// * html_data (String): Data which will be injected into {{TAG}}
pub fn add_component(component_tag: String, html_data: String) -> String {
  // Q. Why "&" is needed here?
  // Generate component tag.
  let css_tag: String = component_tag.to_uppercase() + &String::from("_CSS");
  let html_tag: String = component_tag.to_uppercase() + &String::from("_HTML");

  // Generate file path.
  let css_path = String::from("./templates/components/") + &component_tag.to_lowercase() + &String::from(".css");
  let css_loaded = read_file(&css_path);

  let html_path = String::from("./templates/components/") + &component_tag.to_lowercase() + &String::from(".html");
  let html_loaded = read_file(&html_path);

  // Inject html/css.
  let html_data = html_data.replace(css_tag.as_str(), &css_loaded);
  let html_data = html_data.replace(html_tag.as_str(), &html_loaded);

  return html_data;
}