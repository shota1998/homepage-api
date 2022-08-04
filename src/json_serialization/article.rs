use serde::Deserialize;
use serde::Serialize;
use crate::models::article::article::Article as ModelArticle;

#[derive(Deserialize, Serialize, Clone, Debug)]
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

  // todo: move to trait with new()
  pub fn new_by_model(article_model: &ModelArticle) -> Article {
    return Article {
      id:    article_model.id.clone(),
      title: article_model.title.clone(),
      body:  article_model.body.clone()
    };
  }
}