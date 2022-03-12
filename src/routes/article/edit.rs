use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, Responder};
use crate::database::establish_connection;
use crate::json_serialization::article::Article;
use crate::json_serialization::editing_article::EditingArticle;
use crate::json_serialization::editing_article_without_article_id::EditingArticleWithoutArticleId;
use crate::models::article::editing_article::EditingArticle as Model_EditingArticle;
use crate::models::article::article::Article as Model_Article;
use crate::schema::articles;
use crate::schema::editing_articles;

// todo: test this function
/// Reflect an editing_article to an article in article table.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticleWithoutArticleId>): This serialize the JSON body.
/// 
///  # Returns
///  (HttpResponse): Response body.
pub async fn edit_article(editing_article: web::Json<EditingArticleWithoutArticleId>) -> impl Responder {
  let connection = establish_connection();

  // Extract info from json.
  let id_ref:    &i32    = &editing_article.id.clone();
  let title_ref: &String = &editing_article.title.clone();
  let body_ref:  &String = &editing_article.body.clone();

  // todo: DRY.
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

  // Reflect edits to the article table.
  let filtered_article = articles::table
                        .filter(articles::columns::id.eq(editing_article_model.article_id));

  let article_model  = diesel::update(filtered_article)
                      .set((
                        articles::columns::title.eq(&title_ref),
                        articles::columns::body.eq(&body_ref)
                      ))
                      .get_result::<Model_Article>(&connection)
                      .unwrap();

  // todo : Detect whether eiditing article was created or not.
  // match  update_result {
  //   Ok(_) => HttpResponse::Created().await.unwrap(),
  //   Err(_) => HttpResponse::Conflict().await.unwrap()
  // };
                 
  let article = Article::new(article_model.id.clone(),
                             article_model.title.clone(),
                             article_model.body.clone());

  return article;
}

/// Reflect edits to the editing_article table.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticle>): This serialize the JSON body.
/// 
///  # Returns
///  (Responder): Content of Article.
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

/// Reflesh editing article.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticle>): This serialize the JSON body.
/// 
///  # Returns
///  (Responder): Content of EditingArticle.
// todo 
pub async fn reflesh_editing_article(editing_article: web::Json<EditingArticle>) -> impl Responder {
  let connection = establish_connection();

  // Extract info from json.
  let id_ref:         &i32    = &editing_article.id.clone();
  let article_id_ref: &i32    = &editing_article.article_id.clone();

  // Get an article.
  let article_model = articles::table
                               .filter(articles::columns::id.eq(article_id_ref))
                               .order(articles::columns::id.asc())
                               .get_result::<Model_Article>(&connection)
                               .unwrap();

  // Reflect an article to an editing article: Refresh an editing article.
  let filted_editing_article = editing_articles::table
                                                .filter(editing_articles::columns::id.eq(&id_ref));

  let editing_article_model = diesel::update(filted_editing_article)
                                      .set((
                                        editing_articles::columns::title.eq(article_model.title),
                                        editing_articles::columns::body.eq(article_model.body)
                                      ))
                                      .get_result::<Model_EditingArticle>(&connection)
                                      .unwrap();

  // todo : Detect whether eiditing article was created or not.
  // match  update_result {
  //   Ok(_) => HttpResponse::Created().await.unwrap(),
  //   Err(_) => HttpResponse::Conflict().await.unwrap()
  // };
                 
  let editing_article = EditingArticle::new(editing_article_model.id.clone(),
                                            editing_article_model.article_id.clone(),
                                            editing_article_model.title.clone(),
                                            editing_article_model.body.clone());

  return editing_article;
}