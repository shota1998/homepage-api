use actix_web::{web, Responder};
use crate::diesel;
use diesel::prelude::*;
use serde::{Deserialize};

use crate::database::establish_connection;
use crate::schema::{articles, tmp_articles};
use crate::json_serialization::articles::Articles;
use crate::json_serialization::article::Article;
use crate::models::article::article::Article    as Model_Article;
use crate::models::article::article::TmpArticle as Model_Tmp_Article;

#[derive(Deserialize)]
pub struct RequestBody {
    id: i32,
}

/// Get all saved articles.
///
/// # Arguments
/// * request_body web::Json<RequestBody>: 
///
/// # Returns
/// * (Responder): A an article.
pub async fn get_all_articles() -> impl Responder {
  let connection = establish_connection();
  let article_models = articles::table
        .order(articles::columns::id.asc())
        .load::<Model_Article>(&connection)
        .unwrap();

  let mut article_buffer = Vec::new();

  // Convert model to json serializable structure.
  for article_model in article_models {
    let article = Article::new(article_model.id,
                               article_model.title,
                               article_model.body);

    article_buffer.push(article);
  }

  return Articles::new(article_buffer);
}

/// Get a saved article.
/// Extract by id.
///
/// # Arguments
/// * request_body web::Json<RequestBody>: 
///
/// # Returns
/// * (Responder): A an article.
pub async fn get_article_by_id(request_body: web::Json<RequestBody>) -> impl Responder {
  let connection = establish_connection();
  let article_model = articles::table
        .filter(articles::columns::id.eq(&request_body.id))
        .order(articles::columns::id.asc())
        .load::<Model_Article>(&connection)
        .unwrap();

  let article = Article::new(article_model[0].id.clone(),
                             article_model[0].title.clone(),
                             article_model[0].body.clone());

  return article;
}

/// Get a temporarily saved article for editing.
/// Extract by id.
///
/// # Arguments
/// * request_body web::Json<RequestBody>: 
///
/// # Returns
/// * (Responder): A an article.
pub async fn get_tmp_article_by_id(request_body: web::Json<RequestBody>) -> impl Responder {
  let connection = establish_connection();
  
  // todo : .filter(tmp_articles::columns::article_id.eq(&request_body.id))
  let article_model = tmp_articles::table
        .filter(tmp_articles::columns::id.eq(&request_body.id))
        .order(tmp_articles::columns::id.asc())
        .load::<Model_Tmp_Article>(&connection)
        .unwrap();

  let article = Article::new(article_model[0].id.clone(),
                             article_model[0].title.clone(),
                             article_model[0].body.clone());

  return article;
}