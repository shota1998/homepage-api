use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, Responder};
use crate::database::establish_connection;
use crate::json_serialization::editing_article::EditingArticle;
use crate::json_serialization::editing_article_without_article_id::EditingArticleWithoutArticleId;
use crate::models::article::editing_article::EditingArticle as Model_EditingArticle;
use crate::models::article::article::Article as Model_Article;
use crate::schema::articles;
use crate::schema::editing_articles;

// todo: The query builder should be divided as follows.
//         1: Obtaining an ID
//         2: Getting the content
// todo: Move common processes into models.

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

  // Extract s3 objects url from an "article"/"editing article" using a regular expression.
  use regex::Regex;

  let regex_for_article         = Regex::new(r"\d+").unwrap();
  let regex_for_editing_article = Regex::new(r"\d+").unwrap();

  let article_body:         &str = &article_model.body;
  let editing_article_body: &str = &editing_article_model.body;

  let mut object_urls_in_editing_article: Vec<String> = vec![];

  for object_url in regex_for_editing_article.captures_iter(editing_article_body) {
    println!("{}", &object_url[0]);
    object_urls_in_editing_article.push(String::from(&object_url[0]));
  }

  // Delete s3 objects that are no longer included in the article.
  let mut iter = object_urls_in_editing_article.iter();

  for object_url in regex_for_article.captures_iter(article_body) {
    println!("{}", &object_url[0]);

    if (iter.any(|x| x == &object_url[0])) {
      // delete object
    }
  }

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

/// Reflect edits to the editing_article table.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticleWithoutArticleId>): This serialize the JSON body.
/// 
///  # Returns
///  (Responder): Content of Article.
pub async fn edit_editing_article(editing_article: web::Json<EditingArticleWithoutArticleId>) -> impl Responder {
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

// todo: selct id from article table by article_id of editing_article.
/// Reflesh editing article.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticle>): This serialize the JSON body.
/// 
///  # Returns
///  (Responder): Content of EditingArticle.
// todo 
pub async fn reflesh_editing_article(editing_article: web::Json<EditingArticleWithoutArticleId>) -> impl Responder {
  let connection = establish_connection();

  // Extract info from json.
  let id_ref: &i32 = &editing_article.id.clone();

  // Get the column "article_id" from the table "editing_article".
  let article_id: i32 = editing_articles::table
                                         .filter(editing_articles::id.eq(&id_ref))
                                         .select(editing_articles::article_id)
                                         .first(&connection)
                                         .unwrap();

  // Get an article.
  let article_model = articles::table
                               .filter(articles::columns::id.eq(article_id))
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