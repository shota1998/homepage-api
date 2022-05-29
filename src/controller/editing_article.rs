use diesel::{prelude::*, connection::TransactionManager};
use actix_web::{web, HttpResponse};
use std::env;
use dotenv::dotenv;
use crate::database::establish_connection;
use crate::my_regex::s3::extract_object_keys_to_be_deleted;
use crate::sdk::aws::s3::{client::get_aws_client, delete::delete_objects};
use crate::logic::{article, editing_article};
use crate::json_serialization::editing_article::EditingArticle;
use crate::models::article::editing_article::EditingArticle as Model_EditingArticle;

pub async fn reflect(editing_article_json: web::Json<EditingArticle>) -> HttpResponse {
  dotenv().ok();
  let c  = establish_connection();
  let tm = c.transaction_manager();

  match async {
    // Reflect an editing article to an article.
    let editing_article_model = Model_EditingArticle::new_by_json(&editing_article_json);
    let article_model = article::update(editing_article_model.clone(), &c).await.map_err(|_| ())?;
                        editing_article::update(editing_article_model.clone(), &c).await.map_err(|_| ())?;

    // Delete objects which are no longer used in reflected article.
    let aws_client  = &get_aws_client().unwrap();
    let bucket_name = &env::var("AWS_BUCKET").expect("Missing AWS_BUCKET");
    let object_keys_to_be_deleted: Vec<String> = 
          extract_object_keys_to_be_deleted(&article_model.body, &editing_article_json.body);

    delete_objects(aws_client, bucket_name, object_keys_to_be_deleted).await.map_err(|_| ())
  }
  .await
  {
    //todo: write error messages as constant value which enable us know where and how error occured.
    Ok(_) => match tm.commit_transaction(&c){
        Ok(_)  => return HttpResponse::Ok().json(editing_article_json),
        Err(_) => return HttpResponse::InternalServerError().await.unwrap(),
      },
    Err(_) => match tm.rollback_transaction(&c) {
        Ok(_)  => return HttpResponse::InternalServerError().await.unwrap(),
        Err(_) => return HttpResponse::InternalServerError().await.unwrap(),
      },
  }
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