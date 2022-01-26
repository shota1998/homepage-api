use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct Article {
  pub id: i32,
  pub title: String,
  // pub date: String,
  // pub genre: String
  pub body: String
}

impl Article {
  /// This function constructs the Articles struct.
  ///
  /// # Arguments
  /// * input_items (Vec<ItemTypes>): the to do items super structs to be packaged
  ///
  /// # Returns
  /// * (Articles): package struct
  pub fn new(id: i32, title: String, body: String) -> Article {
    return Article {
      id    : id,
      title : title,
      body  : body
    }
  }
}