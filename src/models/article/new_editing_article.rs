use crate::schema::editing_articles;

#[derive(Insertable)]
#[derive(Clone)]
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
}