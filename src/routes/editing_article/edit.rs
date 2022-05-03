use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, Responder};
use crate::database::establish_connection;
use crate::json_serialization::editing_article::EditingArticle;
use crate::json_serialization::editing_article_without_article_id::EditingArticleWithoutArticleId;
use crate::models::article::editing_article::EditingArticle as Model_EditingArticle;
use crate::schema::editing_articles;

/// Reflect edits to the editing_article table.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticleWithoutArticleId>): This serialize the JSON body.
/// 
///  # Returns
///  (Responder): Content of Article.
pub async fn edit(editing_article: web::Json<EditingArticleWithoutArticleId>) -> impl Responder {
  let connection = establish_connection();

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
