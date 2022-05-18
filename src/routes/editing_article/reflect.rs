use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::connection::TransactionManager;
use actix_web::{web, HttpResponse};
use std::env;
use dotenv::dotenv;
use aws_sdk_s3::Error as S3Error;
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
// pub async fn reflect(editing_article: web::Json<EditingArticleWithoutArticleId>) -> Result {
pub async fn reflect(editing_article: web::Json<EditingArticleWithoutArticleId>) -> HttpResponse {
  // Extract info from json.
  let id_ref:    &i32    = &editing_article.id.clone();
  let title_ref: &String = &editing_article.title.clone();
  let body_ref:  &String = &editing_article.body.clone();

  let mut article_model:         Model_Article = Model_Article{body: "1".to_owned(), id: 2, title: "3".to_owned()};
  let mut editing_article_model: Model_EditingArticle = Model_EditingArticle{body: "1".to_owned(), id: 2, article_id: 3, title: "3".to_owned()};
  let mut editing_article:       EditingArticle = EditingArticle{body: "1".to_owned(), id: 2, article_id: 3, title: "3".to_owned()};

  // Begin transaction.
  let c  = establish_connection();
  let tm = c.transaction_manager();

  // ---------------------
  // Reflect article.
  // ---------------------
  match async {
    tm.begin_transaction(&c)?;

    // Reflect edits to the editing_article table.
    let filtered_editing_article = editing_articles::table
                                   .filter(editing_articles::columns::id.eq(&id_ref));

    editing_article_model = diesel::update(filtered_editing_article)
                              .set((
                                editing_articles::columns::title.eq(&title_ref),
                                editing_articles::columns::body .eq(&body_ref)
                              ))
                              .get_result::<Model_EditingArticle>(&c)?;

    // Reflect edits to the article table.
    let filtered_article = articles::table
                           .filter(articles::columns::id.eq(editing_article_model.article_id));

    article_model = diesel::update(filtered_article)
                      .set((
                        articles::columns::title.eq(&title_ref),
                        articles::columns::body.eq(&body_ref)
                      ))
                      .get_result::<Model_Article>(&c)?;

    // todo: move this to trait.
    editing_article = EditingArticle::new(
                        editing_article_model.id.clone(),
                        editing_article_model.article_id.clone(),
                        editing_article_model.title.clone(),
                        editing_article_model.body.clone()
                      );

    Ok::<_, Error>(())
  }
  .await
  {
    Ok(_)  => (),
    Err(_) => match tm.rollback_transaction(&c) {
        Ok(_)  => return HttpResponse::InternalServerError().await.unwrap(),
        Err(_) => return HttpResponse::InternalServerError().await.unwrap(),
      },
  };

  // ---------------------
  // Delete S3 objects.
  // ---------------------
  // todo: 2022/05/10 22:50
  // I don't know how to handle different types of errors at the same time.
  // Instead, handle different types of errors separately.
  match async {
    dotenv().ok();

    let aws_client  = &get_aws_client().unwrap();
    let bucket_name = &env::var("AWS_BUCKET").expect("Missing AWS_BUCKET");
    let object_keys_to_be_deleted: Vec<String> = 
          extract_object_keys_to_be_deleted(&article_model.body, &editing_article_model.body);

    delete_objects(
      aws_client, 
      bucket_name, 
      object_keys_to_be_deleted
    ).await?;

    Ok::<EditingArticle, S3Error>(editing_article)
  }
  .await
  {
    Ok(editing_article) => match tm.commit_transaction(&c){
        Ok(_)  => return HttpResponse::Ok().json(editing_article),
        Err(_) => return HttpResponse::InternalServerError().await.unwrap(),
      },
    Err(_) => match tm.rollback_transaction(&c) {
        Ok(_)  => return HttpResponse::InternalServerError().await.unwrap(),
        Err(_) => return HttpResponse::InternalServerError().await.unwrap(),
      },
  };

  // // Reflect edits to the editing_article table.
  // let filtered_editing_article = editing_articles::table
  //                               .filter(editing_articles::columns::id.eq(&id_ref));

  // let editing_article_model = diesel::update(filtered_editing_article)
  //                             .set((
  //                               editing_articles::columns::title.eq(&title_ref),
  //                               editing_articles::columns::body .eq(&body_ref)
  //                             ))
  //                             .get_result::<Model_EditingArticle>(&c)
  //                             .unwrap();

  // // Reflect edits to the article table.
  // let filtered_article = articles::table
  //                       .filter(articles::columns::id.eq(editing_article_model.article_id));

  // let article_model = diesel::update(filtered_article)
  //                   .set((
  //                     articles::columns::title.eq(&title_ref),
  //                     articles::columns::body.eq(&body_ref)
  //                   ))
  //                   .get_result::<Model_Article>(&c)
  //                   .unwrap();

  // // todo: move this to trait.
  // let editing_article = EditingArticle::new(editing_article_model.id.clone(),
  //                         editing_article_model.article_id.clone(),
  //                         editing_article_model.title.clone(),
  //                         editing_article_model.body.clone());

  // // Delete S3 objects.
  // dotenv().ok();
  
  // let aws_client  = &get_aws_client().unwrap();
  // let bucket_name = &env::var("AWS_BUCKET").expect("Missing AWS_BUCKET");
  // let object_keys_to_be_deleted: Vec<String> = 
  //   extract_object_keys_to_be_deleted(&article_model.body, &editing_article_model.body);

  
  // let delete_succeeded = delete_objects(
  //                             aws_client, 
  //                             bucket_name, 
  //                             object_keys_to_be_deleted
  //                         ).await.unwrap();
}

#[cfg(test)]
mod routes_edting_article_reflect {
  use super::*;
  use diesel::connection::Connection;
  use diesel::connection::TransactionManager;
  use diesel::result::Error;
  use diesel::prelude::*;
  use crate::database::establish_connection;
  use crate::schema::articles;
  use crate::models::article::new_article::NewArticle as Model_NewArticle;
  use crate::models::article::article::Article        as Model_Article;
  use crate::routes::article::create::create;

  fn create_editing_article_without_article_id() {

  }

  #[actix_web::test]
  async fn test_reflect() {
    let connection = establish_connection();

    connection.test_transaction::<_, Error, _>(|| {
      //todo
      // Mock delte_object()
      //   1: return true
      //   2: return panic

      create();
      edit();
      
      let result = reflect(editing_article: web::Json<EditingArticleWithoutArticleId>);

      let article         = get_article();
      let editing_article = get_editing_article();

      assert_eq!(article, editing_article);
      assert_eq!(result, editing_article);
      Ok(())
    });
  }

  // fn hoge(connection: &PgConnection) -> Result<Vec<String>, Error> {
  //   let article_title = articles::table
  //                           .select(articles::columns::title)
  //                           .filter(articles::columns::title.eq("test"))
  //                           .load::<String>(connection)
  //                           .unwrap();
  //   Ok(article_title)
  // }

  // #[test]
  // fn rollback() {
  //   let connection = establish_connection();

  //   connection.test_transaction::<_, Error, _>(|| {
  //     //todo
  //     let c  = establish_connection();
  //     let tm = c.transaction_manager();
  //     tm.begin_transaction(&c)?;

  //     let title : String = "test".to_owned();
  //     let body  : String = "test".to_owned();

  //     let new_article_model = Model_NewArticle::new(
  //                               title.clone(), 
  //                               body.clone()
  //                             );

  //     diesel::insert_into(articles::table)
  //       .values(&new_article_model)
  //       .get_result::<Model_Article>(&connection)
  //       .unwrap();

  //     let article_title = articles::table
  //                         .select(articles::columns::title)
  //                         .filter(articles::columns::title.eq("test"))
  //                         .load::<String>(&connection)
  //                         .unwrap();

  //     assert_eq!(vec!["test"], article_title);

  //     tm.commit_transaction(&c)?;

  //     Ok(())
  //   });

  //   assert_eq!(vec!["test"], hoge(&connection).unwrap());
  // }
}