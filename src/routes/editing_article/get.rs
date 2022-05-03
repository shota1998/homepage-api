use actix_web::{web, Responder};
use crate::diesel;
use diesel::prelude::*;
use serde::{Deserialize};

use crate::database::establish_connection;
use crate::schema::editing_articles;
use crate::json_serialization::editing_articles::EditingArticles;
use crate::json_serialization::editing_article::EditingArticle;
use crate::models::article::editing_article::EditingArticle as Model_EditingArticle;

#[derive(Deserialize)]
pub struct RequestBody {
    id: i32,
}

#[derive(Deserialize)]
pub struct RequestParameter {
    id: i32,
}

/// Get all editing articles.
///
/// # Arguments
/// * request_body web::Json<RequestBody>: 
///
/// # Returns
/// * (Responder): A an article.
pub async fn get_all() -> impl Responder {
  let connection = establish_connection();

  let editing_article_models = editing_articles::table
                              .order(editing_articles::columns::id.asc())
                              .load::<Model_EditingArticle>(&connection)
                              .unwrap();

  let mut editing_article_buffer = Vec::new();

  // Convert model to json serializable structure.
  for editing_article_model in editing_article_models {
    let editing_article = EditingArticle::new(editing_article_model.id,
                                              editing_article_model.article_id,
                                              editing_article_model.title,
                                              editing_article_model.body);

    editing_article_buffer.push(editing_article);
  }

  return EditingArticles::new(editing_article_buffer);
}

/// Get a editing article.
/// Extract by id.
///
/// # Arguments
/// * request_body web::Json<RequestBody>: 
///
/// # Returns
/// * (Responder): An editing article.
pub async fn get_by_id(request_body: web::Json<RequestBody>) -> impl Responder {
  let connection = establish_connection();
  
  let editing_articles_model = editing_articles::table
                                                .filter(editing_articles::columns::id.eq(&request_body.id))
                                                .order(editing_articles::columns::id.asc())
                                                .load::<Model_EditingArticle>(&connection)
                                                .unwrap();

  let editing_article = EditingArticle::new(editing_articles_model[0].id.clone(),
                                            editing_articles_model[0].article_id.clone(),
                                            editing_articles_model[0].title.clone(),
                                            editing_articles_model[0].body.clone());

  return editing_article;
}

/// Get a editing article.
/// Extract by article_id.
///
/// # Arguments
/// * request_parameter web::Query<RequestParameter>:
///
/// # Returns
/// * (Responder): An editing article.
// pub async fn get_editing_article_by_article_id(request_body: web::Json<RequestBody>) -> impl Responder {
pub async fn get_by_article_id(request_parameter: web::Query<RequestParameter>) -> impl Responder {
  let connection = establish_connection();
  
  let editing_articles_model = editing_articles::table
                                                .filter(editing_articles::columns::article_id.eq(&request_parameter.id))
                                                .order(editing_articles::columns::id.asc())
                                                .load::<Model_EditingArticle>(&connection)
                                                .unwrap();

  let editing_article = EditingArticle::new(editing_articles_model[0].id.clone(),
                                            editing_articles_model[0].article_id.clone(),
                                            editing_articles_model[0].title.clone(),
                                            editing_articles_model[0].body.clone());

  return editing_article;
}