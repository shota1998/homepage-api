use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::json_serialization::new_article::NewArticle;
use crate::models::article::new_article::NewArticle                as Model_NewArticle;
use crate::models::article::article::Article                       as Model_Article;
use crate::models::article::new_editing_article::NewEditingArticle as Model_NewEditingArticle;
use crate::schema::articles;
use crate::schema::editing_articles;

/// This creates an article and saves it to DB.
///
/// # Arguments
/// * req(HttpRequest): the HTTP request passed into the view
/// 
/// # Returns
/// * (impl Responder): message to be sent back to the user. 
pub async fn create(new_article: web::Json<NewArticle>) -> HttpResponse {
  let connection = establish_connection();

  let title : String = new_article.title.clone();
  let body  : String = new_article.body.clone();

  let new_article   = Model_NewArticle::new(title.clone(), body.clone());
  let insert_result = diesel::insert_into(articles::table)
                             .values(&new_article)
                             .get_result::<Model_Article>(&connection);

  // todo : set insert_result::id on article_id;
  // todo : add article_id to tmp_article table;
  // todo : add table "editing article", then use it instead of "editing article" which will be used later;
  let editing_article       = Model_NewEditingArticle::new(insert_result.unwrap().id.clone(), title.clone(), body.clone());
  let insert_result_editing = diesel::insert_into(editing_articles::table)
                                     .values(&editing_article)
                                     .execute(&connection);
  
  // Storing was succeeded or not.
  match insert_result {
    Ok(_)  => HttpResponse::Created().await.unwrap(),
    Err(_) => HttpResponse::Conflict().await.unwrap()
  };

  match insert_result_editing {
    Ok(_)  => HttpResponse::Created().await.unwrap(),
    Err(_) => HttpResponse::Conflict().await.unwrap()
  }
}