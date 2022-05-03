use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};
use std::env;
use dotenv::dotenv;
use crate::database::establish_connection;
use crate::json_serialization::editing_article::EditingArticle;
use crate::json_serialization::editing_article_without_article_id::EditingArticleWithoutArticleId;
use crate::models::article::editing_article::EditingArticle as Model_EditingArticle;
use crate::models::article::article::Article as Model_Article;
use crate::schema::articles;
use crate::schema::editing_articles;
use crate::my_regex::s3::extract_object_keys_to_be_deleted;
use crate::sdk::aws::s3::client::get_aws_client;
use crate::sdk::aws::s3::delete::delete_objects;

/// Reflect an editing_article to an article in article table.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticleWithoutArticleId>): This serialize the JSON body.
/// 
///  # Returns
///  (HttpResponse): Response body.
// pub async fn reflect(editing_article: web::Json<EditingArticleWithoutArticleId>) -> impl Responder {
  pub async fn reflect(editing_article: web::Json<EditingArticleWithoutArticleId>) -> HttpResponse {
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

  // todo: check whther update was succeaded or not.

  // ------------------------------------------
  // Delete S3 objects.
  // ------------------------------------------
  dotenv().ok();

  let aws_client  = &get_aws_client().unwrap();
  let bucket_name = &env::var("AWS_BUCKET").expect("Missing AWS_BUCKET");
  let object_keys_to_be_deleted: Vec<String> = 
    extract_object_keys_to_be_deleted(&article_model.body, &editing_article_model.body);

  let delete_succeeded = delete_objects(aws_client, 
                                        bucket_name, 
                                        object_keys_to_be_deleted
                                       ).await.unwrap();

  if delete_succeeded != true {
    return HttpResponse::InternalServerError().await.unwrap();
  }

                 
  let editing_article = EditingArticle::new(editing_article_model.id.clone(),
                                            editing_article_model.article_id.clone(),
                                            editing_article_model.title.clone(),
                                            editing_article_model.body.clone());

  // return editing_article;
  return HttpResponse::Ok().json(editing_article);
}