use crate::schema::editing_articles;

#[derive(Queryable, Identifiable, Associations)]
#[table_name="editing_articles"]
pub struct EditingArticle {
  pub id         : i32,
  pub article_id : i32,
  pub title      : String,
  pub body       : String
}

impl EditingArticle {
  /// Creates a new instance of the EditingArticle struct.
  ///
  /// # Arguments
  /// * id (i32) : The id of the article.
  /// * article_id (i32) : The article_id of the article.
  /// * title (String) : The title of the article.
  /// * body  (String) : The body of the article.
  ///
  /// # Returns
  /// (NewUser) :
  pub fn new(id: i32, article_id: i32, title: String, body: String) -> EditingArticle {
    return EditingArticle {id, article_id, title, body};
  }
}