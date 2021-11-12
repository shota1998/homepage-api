use crate::diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use crate::schema::articles;
use crate::models::article::{
  new_article::NewArticle,
  article::Article,
  editing_article::EditingArticle
};

/// Get an article by id.
///
/// # Arguments
/// * id(i32): An article's id.
/// * c(&PgConnection): Connection with postgress.
///
/// # Returns
/// * (Result<Article, Error>): A an article.
pub async fn get_by_id(id: i32, c: &PgConnection) -> Result<Article, Error> {

  articles::table
           .filter(articles::columns::id.eq(&id))
           .order(articles::columns::id.asc())
           .get_result::<Article>(c)
}

/// Creates an article.
///
/// # Arguments
/// * new_article(NewArticle): A new article model.
/// * c(&PgConnection): Connection with postgress.
/// 
/// # Returns
/// * (Result<Article, Error>): An artcle model. 
pub async fn create(new_article: NewArticle, c: &PgConnection) -> Result<Article, Error> {

  diesel::insert_into(articles::table)
         .values(&new_article)
         .get_result::<Article>(c)
}

/// Update an article.
/// 
/// # Arguments
/// * editing_article(EditingArticle): An editing article model.
/// * c(&PgConnection): Connection with postgress.
/// 
/// # Returns
/// * (Result<Article, Error>): An artcle model. 
pub async fn update(editing_article: EditingArticle, c: &PgConnection) -> Result<Article, Error> {
  
  let filtered_article = articles::table
                         .filter(articles::columns::id.eq(
                           editing_article.article_id
                          ));

  diesel::update(filtered_article)
         .set((
           articles::columns::title.eq(&editing_article.title.clone()),
           articles::columns::body.eq(&editing_article.body.clone())
         ))
         .get_result::<Article>(c)
}

#[cfg(test)]
mod logic_article {
  use super::*;
  use diesel::pg::PgConnection;
  use crate::database::establish_connection;
  use crate::models::article::{
    new_article::NewArticle,
    article::Article,
    editing_article::EditingArticle
  };
  use crate::test_utils;

  fn create_new_article_model() -> NewArticle {
    NewArticle::new("test title".to_owned(), "test body".to_owned())
  }

  fn create_editing_article_model(article_id: i32) -> EditingArticle {
    EditingArticle::new(1, article_id,"test title".to_owned(), "test body".to_owned())
  }

  // todo: test get_by_id and get_all

  //todo: original assert function.
  #[actix_web::test]
  async fn test_create() {
    let c = test_utils::database::establish_test_connection();

    let new_article_model: NewArticle = create_new_article_model();
    let article_model:     Article    = create(new_article_model.clone(), &c).await.unwrap();
    
    assert_eq!(new_article_model.title, article_model.title);
    assert_eq!(new_article_model.body,  article_model.body);
  }

  #[actix_web::test]
  async fn test_update() {
    let c = test_utils::database::establish_test_connection();

    let new_article_model: NewArticle = create_new_article_model();
    let article_model:     Article    = create(new_article_model.clone(), &c).await.unwrap();

    let editing_article_model: EditingArticle = create_editing_article_model(article_model.id.clone());
    let updated_article_model: Article        = update(editing_article_model.clone(), &c).await.unwrap();

    assert_eq!(editing_article_model.title, updated_article_model.title);
    assert_eq!(editing_article_model.body,  updated_article_model.body);
  }
}