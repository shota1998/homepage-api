use crate::diesel;
use crate::diesel::prelude::*;
use actix_web::{web, HttpResponse };
use serde::{Deserialize, Serialize};

use crate::database::establish_connection;
use crate::models::article::article::Article as Model_Article;
use crate::models::article::editing_article::EditingArticle as Model_EditingArticle;
use crate::schema::articles;
use crate::schema::editing_articles;

#[derive(Deserialize)]
pub struct RequestBody {
    id: i32,
}

#[derive(Serialize)]
pub struct ResponseBody {
  message: String
}

/// Delete an article and an editing article.
///
/// # Arguments
/// * request_body (web::Json<RequestBody>):
///
/// # Returns
/// (HttpResponse):
pub async fn delete(request_body: web::Json<RequestBody>) -> HttpResponse {
  let connection = establish_connection();

  // Delete editing article.
  let editing_articles = editing_articles::table
                         .filter(editing_articles::columns::article_id.eq(&request_body.id))
                         .load::<Model_EditingArticle>(&connection)
                         .unwrap();
                          
  let delete_result_editing = diesel::delete(&editing_articles[0])
                              .execute(&connection);

  // Delete article.
  let articles = articles::table
                 .filter(articles::columns::id.eq(&request_body.id))
                 .load::<Model_Article>(&connection)
                 .unwrap();
                          
  let delete_result = diesel::delete(&articles[0])
                      .execute(&connection);

  match  delete_result {
    Ok(_)  => HttpResponse::Ok().json(ResponseBody {
                message: String::from("Delete an article succeded.")
              }),

    Err(_) => HttpResponse::Conflict().await.unwrap()
  }
}