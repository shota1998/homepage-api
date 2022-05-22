use crate::diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::articles;
use crate::models::article::{
  editing_article::EditingArticle,
  article::Article
};

/// Update an article.
/// 
/// # Arguments
/// * editing_article (web::Json<EditingArticleWithoutArticleId>): This serialize the JSON body.
/// 
///  # Returns
///  (Responder): Content of Article.
pub async fn update(editing_article: EditingArticle, c: &PgConnection) -> Article {
  
  let filtered_article = articles::table
                         .filter(articles::columns::id.eq(editing_article.article_id));

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
  use diesel::result::Error;
  use crate::database::establish_connection;

  #[actix_web::test]
  async fn test_update() {
    let c = establish_connection();

    c.test_transaction::<_, Error, _>(|| {
      // todo: create article
      // todo: update editing_article, but use mock.
      // todo: get editing_article
      let editing_article = editing_article::get(editing_article_id);
      
      assert_eq!(true, true);
      update(editing_article, &c);

      Ok(())
    })
  }
}