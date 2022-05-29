use crate::schema::editing_articles;
use crate::json_serialization::editing_article::EditingArticle as Serial_EditingArticle;
use actix_web::web;

#[derive(Queryable, Identifiable, Associations)]
#[derive(Clone)]
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
    return EditingArticle {
      id:         id,
      article_id: article_id,
      title:      title,
      body:       body
    };
  }

  // todo: move to trait with new()
  pub fn new_by_json(editing_article_model: &web::Json<Serial_EditingArticle>) -> EditingArticle {
    return EditingArticle {
      id:         editing_article_model.id.clone(),
      article_id: editing_article_model.article_id.clone(),
      title:      editing_article_model.title.clone(),
      body:       editing_article_model.body.clone()
    };
  }
}