use diesel::prelude::*;
use actix_web::{web, HttpResponse};
use crate::database::establish_connection;
use crate::logic::article;
use crate::json_serialization::editing_article::EditingArticle;
use crate::models::article::editing_article::EditingArticle as Model_EditingArticle;

// pub async fn reflect(editing_article: web::Json<EditingArticle>) -> HttpResponse {
pub async fn reflect(editing_article: web::Json<EditingArticle>) {
  // Begin transaction.
  let c  = establish_connection();
  let tm = c.transaction_manager();

  // todo: convert editing_article to model from seriarized json.
  let editing_article_model = Model_EditingArticle::new_by_json(editing_article);
  let article_model = article::update(editing_article_model, &c);
  
  // let editing_article_model = editing_article_model::update();
  // let editing_article       = editing_article::new(editing_article_model);
  // error handling.
  // delete object.
  // errro handling.
}

#[cfg(test)]
mod controller_edting_article_reflect {
  use super::*;
  // todo: move establish_test_connection to here.
  // use crate::test::utilts;

  // #[actix_web::test]
  // async fn test_reflect() {
  //   let c = establish_test_connection();    

  //   let editing_article = EditingArticle::new();
  //   // todo: use http request.
  //   let reflected_editing_article = http::request(reflect(editing_article));
  //   let article = Article::get(editing_article.article_id);

  //   // todo: move these test to a function. Then use three times.
  //   assert_eq!(editing_article.article_id, article.id);
  //   assert_eq!(editing_article.title,      article.title);
  //   assert_eq!(editing_article.body,       article.body);
  //   // todo: add test.
  // }
}