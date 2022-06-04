use crate::schema::editing_articles;
use crate::models::article::article::Article as ModelArticle;

#[derive(Insertable, Clone)]
#[table_name="editing_articles"]
pub struct NewEditingArticle {
  pub article_id : i32,
  pub title      : String,
  pub body       : String
}

impl NewEditingArticle {
  /// Creates a new instance of the NewEditingArticle struct.
  ///
  /// # Arguments
  /// * article_id (i32) : The article_id of the article.
  /// * title (String) : The title of the article.
  /// * body  (String) : The body of the article.
  ///
  /// # Returns
  /// (NewUser) :
  pub fn new(article_id: i32, title: String, body: String) -> NewEditingArticle {
    return NewEditingArticle {article_id, title, body};
  }

  // todo: move to trait with new()
  pub fn new_by_article_model(article_model: &ModelArticle) -> NewEditingArticle {
    return NewEditingArticle {
      article_id: article_model.id.clone(),
      title:      article_model.title.clone(),
      body:       article_model.body.clone()
    };
  }
}