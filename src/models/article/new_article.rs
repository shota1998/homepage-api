use crate::schema::articles;

#[derive(Insertable)]
#[derive(Clone)]
#[table_name="articles"]
pub struct NewArticle {
  pub title : String,
  pub body  : String
}

impl NewArticle {
  /// Creates a new instance of the NewArticle struct.
  ///
  /// # Arguments
  /// * title (String): The title of the article.
  /// * body  (String): The body of the article.
  ///
  /// # Returns
  /// (NewUser):
  pub fn new(title: String, body: String) -> NewArticle {
    return NewArticle {title, body};
  }
}