use crate::diesel;

use diesel::prelude::*;
use actix_web::{web, HttpResponse};
// use actix_web::HttpRequest;
// use actix_web::Responder;

use crate::database::establish_connection;
// use crate::models::article::article::Article;
use crate::json_serialization::article::Article;
use crate::models::article::new_article::NewArticle;
use crate::schema::articles;
// use crate::auth::jwt::JwtToken;

// use super::utils::return_state;

/// This creates an article and saves it to DB.
///
/// # Arguments
/// * req(HttpRequest): the HTTP request passed into the view
/// 
/// # Returns
/// * (impl Responder): message to be sent back to the user. 
pub async fn create(new_article: web::Json<Article>) -> HttpResponse {
  let connection    = establish_connection();
  let title: String = new_article.title.clone();
  let body : String = new_article.body.clone();
  let new_article = NewArticle::new(title, body);

  let insert_result = diesel::insert_into(articles::table)
                              .values(&new_article)
                              .execute(&connection);
  
  // Storing was succeeded or not.
  match  insert_result {
    Ok(_) => HttpResponse::Created().await.unwrap(),
    Err(_) => HttpResponse::Conflict().await.unwrap()
  } 
}