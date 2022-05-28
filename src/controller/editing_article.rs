use diesel::{prelude::*, result::Error, connection::TransactionManager};
use actix_web::{web, HttpResponse};
use std::env;
use dotenv::dotenv;
use aws_sdk_s3::Error as S3Error;
use crate::database::establish_connection;
use crate::my_regex::s3::extract_object_keys_to_be_deleted;
use crate::sdk::aws::s3::{client::get_aws_client, delete::delete_objects};
use crate::logic::{article, editing_article};
use crate::json_serialization::editing_article::EditingArticle;
use crate::models::article::{
  article::Article as Model_Article,
  editing_article::EditingArticle as Model_EditingArticle
};

// todo: split to each function.
// 1: reflect editing article to article
// 2: delete object from reflected article
// 3: crate helper which does error handling
//    hoge (f: fn) {
//       match f {
//           ....
//        }
//     }
pub async fn reflect(editing_article: web::Json<EditingArticle>) -> HttpResponse {
  let c  = establish_connection();
  let tm = c.transaction_manager();
  let mut article_model         = Model_Article::new();
  let mut editing_article_model = Model_EditingArticle::new_by_json(editing_article);

  // Reflect an editing article to an article.
  match async {
    // todo: return result instead of article/editing article.
    article_model         = article::update(editing_article_model.clone(), &c);
    editing_article_model = editing_article::update(editing_article_model.clone(), &c).await;
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

  // Delete S3 objects.
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

    // Ok::<EditingArticle, S3Error>(editing_article)
    Ok::<_, S3Error>(())
  }
  .await
  {
    Ok(_) => match tm.commit_transaction(&c){
        Ok(_)  => return HttpResponse::Ok().json(editing_article),
        Err(_) => return HttpResponse::InternalServerError().await.unwrap(),
      },
    Err(_) => match tm.rollback_transaction(&c) {
        Ok(_)  => return HttpResponse::InternalServerError().await.unwrap(),
        Err(_) => return HttpResponse::InternalServerError().await.unwrap(),
      },
  };
}

#[cfg(test)]
mod controller_edting_article_reflect {
  use super::*;
  // todo: move establish_test_connection to here.
  // use crate::test::utilts;

  // #[actix_web::test]
  // async fn test_reflect() {
  //   let c = establish_test_connection();    

  //   let editing_article = EditingArticle::new();
  //   // todo: use http request.
  //   let reflected_editing_article = http::request(reflect(editing_article));
  //   let article = Article::get(editing_article.article_id);

  //   // todo: move these test to a function. Then use three times.
  //   assert_eq!(editing_article.article_id, article.id);
  //   assert_eq!(editing_article.title,      article.title);
  //   assert_eq!(editing_article.body,       article.body);
  //   // todo: add test.
  // }
}