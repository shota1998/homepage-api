use crate::diesel;
use crate::diesel::prelude::*;
use actix_web::{web, HttpResponse };
use serde::{Deserialize, Serialize};

use crate::database::establish_connection;
use crate::models::article::article::Article as Model_Article;
use crate::schema::articles;

#[derive(Deserialize)]
pub struct RequestBody {
    id: i32,
}

#[derive(Serialize)]
pub struct ResponseBody {
  message: String
}

/// This function deletes a to do item's status.
///
/// # Arguments
/// * to_di_item (web::Json<ToDoItem>): This serializes the JSON body via the ToDoItem struct
///
/// # Returns
/// (HttpResponse): response body to be passed to the viewer.
pub async fn delete(request_body: web::Json<RequestBody>) -> HttpResponse {
  let connection = establish_connection();
  let items = articles::table
              .filter(articles::columns::id.eq(&request_body.id))
              .load::<Model_Article>(&connection)
              .unwrap();
                          
  let delete_result = diesel::delete(&items[0])
                      .execute(&connection);

  // todo : delete editing article.

  match  delete_result {
    Ok(_) => HttpResponse::Ok().json(ResponseBody {
        message: String::from("Delete succeded.")
      }),
    Err(_) => HttpResponse::Conflict().await.unwrap()
  } 
}