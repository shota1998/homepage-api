use crate::schema::articles;
use crate::schema::tmp_articles;

#[derive(Insertable)]
#[table_name="articles"]
pub struct NewArticle {
  pub title : String,
  pub body  : String
}

#[derive(Insertable)]
#[table_name="tmp_articles"]
pub struct TmpNewArticle {
  // pub article_id : i32,
  pub title : String,
  pub body  : String
}

// pub struct NewPost<'a> {
//   pub title: &'a str,
//   pub body: &'a str,
// }

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
  /// Creates a new instance of the NewUser struct.
  /// This is for editing.
  ///
  /// # Arguments
  /// * title (String): The title of the article.
  /// * body  (String): The body of the article.
  ///
  /// # Returns
  /// (NewUser):
  pub fn tmp_new(title: String, body: String) -> TmpNewArticle {
    return TmpNewArticle {title, body};
  }
}