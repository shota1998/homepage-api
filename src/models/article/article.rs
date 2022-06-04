use actix_web::web;
use crate::json_serialization::article::Article as SerialArticle;
use crate::schema::articles;

#[derive(Queryable, Identifiable, Associations, Clone, PartialEq, Debug)]
#[table_name="articles"]
pub struct Article {
  pub id    : i32,
  pub title : String,
  pub body  : String
}

impl Article {
  // todo: move to trait with new()
  pub fn new_by_json(article_model: &web::Json<SerialArticle>) -> Article {
    return Article {
      id:    article_model.id.clone(),
      title: article_model.title.clone(),
      body:  article_model.body.clone()
    };
  }
}