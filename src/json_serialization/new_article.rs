use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewArticle {
  pub title: String,
  // pub date: String,
  // pub genre: String
  pub body: String
}

impl NewArticle {
  /// This function constructs the Articles struct.
  ///
  /// # Arguments
  /// * input_items (Vec<ItemTypes>): the to do items super structs to be packaged
  ///
  /// # Returns
  /// * (Articles): package struct
  pub fn new(title: String, body: String) -> NewArticle {
    return NewArticle {
      title : title,
      body  : body
    }
  }
}