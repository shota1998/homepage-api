use crate::diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use crate::schema::editing_articles;
use crate::models::article::{
  new_editing_article::NewEditingArticle,
  editing_article::EditingArticle
};

/// Creates an editing article.
///
/// # Arguments
/// * req(NewEditingArticle): 
/// * req(&PgConnection): 
/// 
/// # Returns
/// * (Article): Article model.
pub async fn create(new_editing_article: NewEditingArticle, c: &PgConnection) -> EditingArticle {

  diesel::insert_into(editing_articles::table)
         .values(&new_editing_article)
         .get_result::<EditingArticle>(c)
         .unwrap()
}

/// Update an editing article.
/// 
/// # Arguments
/// * req(EditingArticle): 
/// * req(&PgConnection): 
/// 
///  # Returns
///  (EditingArticle): EditingArticle model.
pub async fn update(editing_article: EditingArticle, c: &PgConnection) -> Result<EditingArticle, Error> {
  
  let filted_editing_article = editing_articles::table
                              .filter(editing_articles::columns::id.eq(
                                &editing_article.id
                              ));

  diesel::update(filted_editing_article)
         .set((
           editing_articles::columns::title.eq(&editing_article.title),
           editing_articles::columns::body.eq(&editing_article.body)
         ))
         .get_result::<EditingArticle>(c)
}

#[cfg(test)]
mod logic_editing_article {
  use super::*;
  use diesel::pg::PgConnection;
  use crate::database::establish_connection;
  use crate::models::article::{
    new_article::NewArticle,
    new_editing_article::NewEditingArticle,
    editing_article::EditingArticle
  };
  use crate::logic::article;

  // todo: move to test_utiliry/database.rs
  fn establish_test_connection() -> PgConnection {
    let c = establish_connection();

    match c.begin_test_transaction() {
      Ok(_)  => c,
      Err(_) => panic!()
    }
  }

  fn create_new_article_model() -> NewArticle {
    NewArticle::new("test title".to_owned(), "test body".to_owned())
  }

  fn create_new_editing_article_model(article_id: i32) -> NewEditingArticle {
    NewEditingArticle::new(article_id, "test title".to_owned(), "test body".to_owned())
  }

  #[actix_web::test]
  async fn test_create() {
    let c = establish_test_connection();
    // todo: move test/create_article
    let new_article_model = create_new_article_model();
    let article_model     = article::create(new_article_model, &c).await;
    // todo: move test/create_editing_article
    let new_editing_article_model = create_new_editing_article_model(article_model.id);
    let editing_article_model     = create(new_editing_article_model.clone(), &c).await;
    
    assert_eq!(new_editing_article_model.title, editing_article_model.title);
    assert_eq!(new_editing_article_model.body,  editing_article_model.body);
  }

  #[actix_web::test]
  async fn test_update() {
    let c = establish_test_connection();
    
    // todo: move test/create_article
    let new_article_model = create_new_article_model();
    let article_model     = article::create(new_article_model, &c).await;

    // todo: move test/create_editing_article
    let new_editing_article_model = create_new_editing_article_model(article_model.id);
    let mut editing_article_model = create(new_editing_article_model.clone(), &c).await;
    editing_article_model.title = "edited test title".to_owned();
    
    let updated_editing_article_model: EditingArticle = update(editing_article_model.clone(), &c).await.unwrap();

    assert_eq!(editing_article_model.title, updated_editing_article_model.title);
    assert_eq!(editing_article_model.body,  updated_editing_article_model.body);
  }
}