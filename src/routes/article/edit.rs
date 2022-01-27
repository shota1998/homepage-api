use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse, HttpRequest};

use super::utils::return_state;

use crate::database::establish_connection;
use crate::json_serialization::article::Article;
use crate::schema::articles;
// use crate::auth::jwt::JwtToken;

/// This is function edits a to do item's status.
/// 
/// # Arguments
/// * articles_item (web::Json<ToDoItem>): This serialize the JSON body via the ToDoItem struct
/// * req (HttpRequest): The request being made
/// 
///  # Returns
///  (HttpResponse): response body to be passed to the viewer.
pub async fn edit(article: web::Json<Article>, req: HttpRequest) -> HttpResponse {
  // Extract title info from json.
  let id_ref:    &i32    = &article.id.clone();
  let title_ref: &String = &article.title.clone();
  let body_ref:  &String = &article.body.clone();

  // Extract token from HttpRequest.
  // let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

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
                 
  // Return items json which will be used to check deleted or not.
  // return HttpResponse::Ok().json(return_state(&token.user_id))
  match  update_result {
    Ok(_) => HttpResponse::Created().await.unwrap(),
    Err(_) => HttpResponse::Conflict().await.unwrap()
  } 
}