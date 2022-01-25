use crate::diesel;
use diesel::prelude::*;
use crate::json_serialization::articles::Articles;
use crate::json_serialization::article::Article;
use crate::database::establish_connection;
use crate::schema::articles;
use crate::models::article::article as Model;

/// Gets all the to do items from the DB.
///
/// # Arguments
/// None
/// 
/// # Returns
/// * (ToDoItems): to do items sorted into Done and Pending with count numbers.
pub fn return_state() -> Articles {
  let connection = establish_connection();
  let article_models = articles::table
        .order(articles::columns::id.asc())
        .load::<Model::Article>(&connection)
        .unwrap();

  let mut articles_buffer = Vec::new();

  // Convert model to json serializable structure.
  for article_model in article_models {
    let article = Article::new(article_model.title, article_model.body);
    articles_buffer.push(article);
  }

  // Convert array of Article to Articles which equipped with Responder trait.
  return Articles::new(articles_buffer);
}

// pub fn return_articles_by_id(article_ids: &i32 = null, article_limit: &i32 = null) -> Articles {
//   // SQL
//   return Articles::new();
// }
