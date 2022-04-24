use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone)]
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
  /// * id (i32): 
  /// * title (String): 
  /// * body (String): 
  ///
  /// # Returns
  /// * (Article): 
  pub fn new(id: i32, title: String, body: String) -> Article {
    return Article {
      id    : id,
      title : title,
      body  : body
    }
  }
}