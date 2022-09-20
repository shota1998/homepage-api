use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::json_serialization::new_article::NewArticle;
use crate::json_serialization::article::Article;
use crate::models::article::new_article::NewArticle                as Model_NewArticle;
use crate::models::article::article::Article                       as Model_Article;
use crate::models::article::new_editing_article::NewEditingArticle as Model_NewEditingArticle;
use crate::models::article::editing_article::EditingArticle        as Model_EditingArticle;
use crate::schema::articles;
use crate::schema::editing_articles;

// todo: split this in to logic and controlloer. check arcitecure pattern for actic web.
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

  // Creat an article.
  let new_article_model = Model_NewArticle::new(title.clone(), 
                                                body.clone());

  let article_model = diesel::insert_into(articles::table)
                             .values(&new_article_model)
                             .get_result::<Model_Article>(&connection)
                             .unwrap();

  let article = Article::new(article_model.id.clone(),
                             article_model.title.clone(),
                             article_model.body.clone());
  
  // Create article for edit.
  // ::new_by_model(article_model)
  let editing_article_model = Model_NewEditingArticle::new(article.id, 
                                                           title.clone(), 
                                                           body.clone());
                                                           
  let insert_result_editing = diesel::insert_into(editing_articles::table)
                                     .values(&editing_article_model)
                                     .get_result::<Model_EditingArticle>(&connection);

  // todo : Detect whether eiditing article was created or not.
  // todo: rollback
  match insert_result_editing {
    Ok(_)  => HttpResponse::Ok().json(article),
    Err(_) => HttpResponse::Conflict().await.unwrap()
  }

  // return article;
}