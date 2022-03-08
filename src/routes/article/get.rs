use actix_web::{web, Responder};
use crate::diesel;
use diesel::prelude::*;
use serde::{Deserialize};

use crate::database::establish_connection;
use crate::schema::{articles, editing_articles};
use crate::json_serialization::articles::Articles;
use crate::json_serialization::article::Article;
use crate::json_serialization::editing_article::EditingArticle;
use crate::models::article::article::Article                as Model_Article;
use crate::models::article::editing_article::EditingArticle as Model_EditingArticle;

#[derive(Deserialize)]
pub struct RequestBody {
    id: i32,
}

/// Get all articles.
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

/// Get an article.
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

/// Get a editing article.
/// Extract by id.
///
/// # Arguments
/// * request_body web::Json<RequestBody>: 
///
/// # Returns
/// * (Responder): A an article.
pub async fn get_editing_article_by_id(request_body: web::Json<RequestBody>) -> impl Responder {
  let connection = establish_connection();
  
  let editing_articles_model = editing_articles::table
                                                .filter(editing_articles::columns::article_id.eq(&request_body.id))
                                                .order(editing_articles::columns::id.asc())
                                                .load::<Model_EditingArticle>(&connection)
                                                .unwrap();

  let editing_article = EditingArticle::new(editing_articles_model[0].id.clone(),
                                            editing_articles_model[0].article_id.clone(),
                                            editing_articles_model[0].title.clone(),
                                            editing_articles_model[0].body.clone());

  return editing_article;
}