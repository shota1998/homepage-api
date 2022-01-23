use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct Article {
  pub title: String,
  // pub date: String,
  // pub genre: String
  pub content: String
}

impl Article {
  /// This function constructs the Articles struct.
  ///
  /// # Arguments
  /// * input_items (Vec<ItemTypes>): the to do items super structs to be packaged
  ///
  /// # Returns
  /// * (Articles): package struct
  pub fn new(title: String, content: String) -> Article {
    return Article {
      title:   title,
      content: content
    }
  }
}