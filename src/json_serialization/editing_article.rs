use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone)]
pub struct EditingArticle {
  pub id         : i32,
  pub article_id : i32,
  pub title      : String,
  pub body       : String
}

impl EditingArticle {
  /// Constructs the EditingArticle struct.
  ///
  /// # Arguments
  /// * id (i32): 
  /// * article_id (i32):
  /// * title (String):
  /// * body (String):
  ///
  /// # Returns
  /// * (EditingArticle): 
  pub fn new(id: i32, article_id: i32, title: String, body: String) -> EditingArticle {
    return EditingArticle {
      id         : id,
      article_id : article_id,
      title      : title,
      body       : body
    }
  }
}