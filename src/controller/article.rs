use diesel::{prelude::*, connection::TransactionManager};
use actix_web::{web, HttpResponse};
use dotenv::dotenv;
use crate::database::establish_connection;
use crate::logic::{
  article,
  editing_article
};
use crate::json_serialization::{
  new_article::NewArticle as SerialNewArticle,
  article::Article as SerialArticle
};
use crate::models::article::{
  new_article::NewArticle as ModelNewArticle,
  new_editing_article::NewEditingArticle as ModelNewEditingArticle
};
use crate::constants;

/// Create an article.
/// # Arguments
/// * article_json(web::Json<Article>): An article json object.
/// # Returns
/// * (HttpResponse): 
pub async fn create(new_article_json: web::Json<SerialNewArticle>) -> HttpResponse {
  // todo: Wrap these three codes in a helper.
  dotenv().ok();
  let c  = establish_connection();
  let tm = c.transaction_manager();

  match async {
    let new_article_model         = ModelNewArticle::new_by_json(&new_article_json);
    let article_model             = article::create(new_article_model.clone(), &c).await.map_err(|_| ())?;
    let article_json              = SerialArticle::new_by_model(&article_model);
    let new_editing_article_model = ModelNewEditingArticle::new_by_article_model(&article_model);
    editing_article::create(new_editing_article_model.clone(), &c).await.map_err(|_| ())?;
    Ok::<SerialArticle, ()>(article_json)
  }
  .await
  {
    Ok(article_json) => match tm.commit_transaction(&c){
        Ok(_)  => return HttpResponse::Ok().json(article_json),
        Err(_) => return HttpResponse::InternalServerError().body(constants::COMMIT_FAILED),
      },
    Err(_) => match tm.rollback_transaction(&c) {
        Ok(_)  => return HttpResponse::InternalServerError().body(constants::REFLECT_FAILED),
        Err(_) => return HttpResponse::InternalServerError().body(constants::ROLLBACK_FAILED),
      },
  }
}