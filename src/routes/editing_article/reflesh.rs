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

// todo: selct id from article table by article_id of editing_article.
/// Reflesh editing article.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticle>): This serialize the JSON body.
/// 
///  # Returns
///  (Responder): Content of EditingArticle.
// todo 
pub async fn reflesh(editing_article: web::Json<EditingArticleWithoutArticleId>) -> impl Responder {
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