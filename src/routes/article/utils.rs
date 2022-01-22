use crate::json_serialization::articles::Articles;

/// Gets all the to do items from the DB.
///
/// # Arguments
/// None
/// 
/// # Returns
/// * (ToDoItems): to do items sorted into Done and Pending with count numbers.
pub fn return_state() -> Articles {
  return Articles::new();
}

// pub fn return_articles_by_id(article_ids: &i32) -> Articles {
//   // SQL
//   return Articles::new();
// }

// pub fn return_articles_with_limit(article_limit: &i32) -> Articles {
//   // SQL
//   return Articles::new();
// }