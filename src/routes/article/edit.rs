use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse };
use crate::database::establish_connection;
use crate::json_serialization::article::Article;
use crate::schema::articles;

/// This is function edits a to do item's status.
/// 
/// # Arguments
/// * articles (web::Json<ToDoItem>): This serialize the JSON body via the ToDoItem struct
/// 
///  # Returns
///  (HttpResponse): Response body.
pub async fn edit(article: web::Json<Article>) -> HttpResponse {
  // Extract info from json.
  let id_ref:    &i32    = &article.id.clone();
  let title_ref: &String = &article.title.clone();
  let body_ref:  &String = &article.body.clone();

  // Edit item data of DB. Change the state to "done".
  let connection = establish_connection();
  let filter_results = articles::table
                      .filter(articles::columns::id.eq(&id_ref));

  let update_result = diesel::update(filter_results)
                      .set((
                        articles::columns::title.eq(&title_ref),
                        articles::columns::body.eq(&body_ref)
                       ))
                      .execute(&connection);
                 
  match  update_result {
    Ok(_) => HttpResponse::Created().await.unwrap(),
    Err(_) => HttpResponse::Conflict().await.unwrap()
  } 
}