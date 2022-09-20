use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone)]
pub struct EditingArticleWithoutArticleId {
  pub id         : i32,
  pub title      : String,
  pub body       : String
}

impl EditingArticleWithoutArticleId {
  /// Constructs the EditingArticleWithoutArticleId struct.
  ///
  /// # Arguments
  /// * id (i32): 
  /// * title (String):
  /// * body (String):
  ///
  /// # Returns
  /// * (EditingArticleWithoutArticleId): 
  pub fn new(id: i32, title: String, body: String) -> EditingArticleWithoutArticleId {
    return EditingArticleWithoutArticleId {
      id         : id,
      title      : title,
      body       : body
    }
  }
}