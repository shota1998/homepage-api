use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use crate::database::establish_connection;
use crate::json_serialization::editing_article::EditingArticle;
use crate::models::article::editing_article::EditingArticle as Model_EditingArticle;
use crate::schema::articles;
use crate::schema::editing_articles;

// todo : convert it to "edti_article"
/// Save an edited article to editing_article tanble. 
/// Reflect it to an article in article table.
/// 
/// # Arguments
/// * editing_article (web::Json<Article>): This serialize the JSON body.
/// 
///  # Returns
///  (HttpResponse): Response body.
pub async fn edit(editing_article: web::Json<EditingArticle>) -> HttpResponse {
  let connection = establish_connection();

  // Extract info from json.
  let id_ref:    &i32    = &editing_article.id.clone();
  let article_id_ref: &i32    = &editing_article.article_id.clone();
  let title_ref: &String = &editing_article.title.clone();
  let body_ref:  &String = &editing_article.body.clone();

  // Store edited article to the editing_article table.
  let filter_results = articles::table
                       .filter(articles::columns::id.eq(&article_id_ref));

  let update_result  = diesel::update(filter_results)
                       .set((
                         articles::columns::title.eq(&title_ref),
                         articles::columns::body.eq(&body_ref)
                       ))
                       .execute(&connection);

  // todo : Detect whether eiditing article was created or not.
  // match  update_result {
  //   Ok(_) => HttpResponse::Created().await.unwrap(),
  //   Err(_) => HttpResponse::Conflict().await.unwrap()
  // };

  // Reflect edited article to the article table.
  let filter_results_editing = editing_articles::table
                               .filter(editing_articles::columns::id.eq(&id_ref));

  let update_result_editing  = diesel::update(filter_results_editing)
                               .set((
                                 editing_articles::columns::title.eq(&title_ref),
                                 editing_articles::columns::body.eq(&body_ref)
                               ))
                               .execute(&connection);
                 
  match  update_result_editing {
    Ok(_) => HttpResponse::Created().await.unwrap(),
    Err(_) => HttpResponse::Conflict().await.unwrap()
  } 
}

/// Reflect edits to the editing_article table.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticle>): This serialize the JSON body.
/// 
///  # Returns
///  (Responder): Content of EditingArticle.
pub async fn edit_editing_article(editing_article: web::Json<EditingArticle>) -> impl Responder {
  let connection = establish_connection();

  // Extract info from json.
  let id_ref:    &i32    = &editing_article.id.clone();
  let title_ref: &String = &editing_article.title.clone();
  let body_ref:  &String = &editing_article.body.clone();

  // Reflect edits to the editing_article table.
  let filted_editing_article = editing_articles::table
                                                .filter(editing_articles::columns::id.eq(&id_ref));

  let editing_article_model = diesel::update(filted_editing_article)
                              .set((
                                editing_articles::columns::title.eq(&title_ref),
                                editing_articles::columns::body.eq(&body_ref)
                              ))
                              .get_result::<Model_EditingArticle>(&connection)
                              .unwrap();

  let article = EditingArticle::new(editing_article_model.id.clone(),
                                    editing_article_model.article_id.clone(),
                                    editing_article_model.title.clone(),
                                    editing_article_model.body.clone());

  return article;
}

// todo 
// pub async fn reflesh_editing_article(editing_article: web::Json<EditingArticle>) -> HttpResponse {

// }