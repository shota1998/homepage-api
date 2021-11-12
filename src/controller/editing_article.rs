use diesel::{prelude::*, connection::TransactionManager};
use actix_web::{web, HttpResponse};
use std::env;
use dotenv::dotenv;
use crate::database::establish_connection;
use crate::my_regex::s3::extract_object_keys_to_be_deleted;
use crate::sdk::aws::s3::{client::get_aws_client, delete::delete_objects};
use crate::logic::{article, editing_article};
use crate::json_serialization::editing_article::EditingArticle;
use crate::models::article::editing_article::EditingArticle as ModelEditingArticle;
use crate::constants;

/// 1: Update editing article with incoming editing article. \
/// 2: Update article with incoming editing article. \
/// 3: Extract image objects from article which are not used in updated article (editing article). \
/// 4: Delete these image objects. \
///
/// # Arguments
/// * editing_article_json(web::Json<EditingArticle>): An editing article json object.
/// 
/// # Returns
/// * (HttpResponse): 
pub async fn reflect(editing_article_json: web::Json<EditingArticle>) -> HttpResponse {
  dotenv().ok();
  let c  = establish_connection();
  let tm = c.transaction_manager();
  tm.begin_transaction(&c);

  match async {
    let article_model         = article::get_by_id(editing_article_json.article_id, &c).await.map_err(|_| ())?;
    let editing_article_model = ModelEditingArticle::new_by_json(&editing_article_json);

    // Reflect an editing article to an article.
    article::update(editing_article_model.clone(), &c).await.map_err(|_| ())?;
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
    Ok(_) => match tm.commit_transaction(&c){
        Ok(_)  => return HttpResponse::Ok().json(editing_article_json),
        Err(_) => return HttpResponse::InternalServerError().body(constants::COMMIT_FAILED),
      },
    Err(_) => match tm.rollback_transaction(&c) {
        Ok(_)  => return HttpResponse::InternalServerError().body(constants::REFLECT_FAILED),
        Err(_) => return HttpResponse::InternalServerError().body(constants::ROLLBACK_FAILED),
      },
  }
}


// todo: create teardown to clear up db.
// #[cfg(test)]
// mod controller_editing_article {
//   #[actix_web::test]
//   async fn test_reflect() {

//     // todo: create create()
//     // todo: test create()

//     // todo: call create
//     // todo: call reflect
//     // todo:  mocke aws function
//     // todo: compare result

//     // todo: check if transaction working whiel this test.
//   }
// }