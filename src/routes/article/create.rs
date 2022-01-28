use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::json_serialization::new_article::NewArticle;
use crate::models::article::new_article::NewArticle as Model_NewArticle;
use crate::schema::articles;

/// This creates an article and saves it to DB.
///
/// # Arguments
/// * req(HttpRequest): the HTTP request passed into the view
/// 
/// # Returns
/// * (impl Responder): message to be sent back to the user. 
pub async fn create(new_article: web::Json<NewArticle>) -> HttpResponse {
  let title : String = new_article.title.clone();
  let body  : String = new_article.body.clone();
  let new_article    = Model_NewArticle::new(title, body);

  let connection = establish_connection();
  let insert_result = diesel::insert_into(articles::table)
                              .values(&new_article)
                              .execute(&connection);
  
  // Storing was succeeded or not.
  match  insert_result {
    Ok(_) => HttpResponse::Created().await.unwrap(),
    Err(_) => HttpResponse::Conflict().await.unwrap()
  } 
}