use crate::schema::articles;
use crate::json_serialization::new_article::NewArticle as SerialNewArticle;
use actix_web::web;

#[derive(Insertable)]
#[derive(Clone)]
#[table_name="articles"]
pub struct NewArticle {
  pub title : String,
  pub body  : String
}

impl NewArticle {
  pub fn new(title: String, body: String) -> NewArticle {
    return NewArticle {title, body};
  }

  // todo: move to trait with new()
  pub fn new_by_json(editing_article_model: &web::Json<SerialNewArticle>) -> NewArticle {
    return NewArticle {
      title:      editing_article_model.title.clone(),
      body:       editing_article_model.body.clone()
    };
  }
}