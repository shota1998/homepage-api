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

// pub fn return_articles_by_id(article_ids: &i32 = null, article_limit: &i32 = null) -> Articles {
//   // SQL
//   return Articles::new();
// }
