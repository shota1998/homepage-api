use crate::diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::articles;
use crate::models::article::{
  new_article::NewArticle,
  article::Article,
  editing_article::EditingArticle
};

/// Creates an article.
///
/// # Arguments
/// * req(HttpRequest): the HTTP request passed into the view
/// 
/// # Returns
/// * (impl Responder): message to be sent back to the user. 
pub async fn create(new_article: NewArticle, c: &PgConnection) -> Article {

  diesel::insert_into(articles::table)
         .values(&new_article)
         .get_result::<Article>(c)
         .unwrap()
}

/// Update an article.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticleWithoutArticleId>): This serialize the JSON body.
/// 
///  # Returns
///  (Responder): Content of Article.
pub async fn update(editing_article: EditingArticle, c: &PgConnection) -> Article {
  
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
         .unwrap()
}

#[cfg(test)]
mod logic_article {
  use super::*;
  use diesel::pg::PgConnection;
  use crate::database::establish_connection;
  use crate::models::article::{
    new_article::NewArticle,
    article::Article,
    // editing_article::EditingArticle
  };

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

  //todo: original assert function.
  #[actix_web::test]
  async fn test_create() {
    let c = establish_test_connection();

    let new_article_model: NewArticle = create_new_article_model();
    let article_model:     Article    = create(new_article_model.clone(), &c).await;
    
    assert_eq!(new_article_model.title, article_model.title);
    assert_eq!(new_article_model.body, article_model.body);
  }

  // #[actix_web::test]
  // async fn test_update() {
  //   let c = establish_connection();

  //   c.test_transaction::<_, Error, _>(|| {
  //     // todo: create article
  //     // todo: update editing_article, but use mock.
  //     // todo: get editing_article
  //     let editing_article = editing_article::get(editing_article_id);
      
  //     update(editing_article, &c);

  //     assert_eq!(true, true);
  //     Ok(())
  //   })
  // }
}