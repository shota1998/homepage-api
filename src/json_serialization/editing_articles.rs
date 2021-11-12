use std::vec::Vec;
use serde::Serialize;
use crate::json_serialization::editing_article::EditingArticle;

/// This struct packages the raw struct fields to package items for JSON serialization.
///
/// # Parameters
/// * editing_articles (Vec<EditingArticle>): 
#[derive(Serialize)]
pub struct EditingArticles {
  pub editing_articles: Vec<EditingArticle>,
}

impl EditingArticles {
  /// Constructs the EditingArticles struct.
  ///
  /// # Arguments
  /// * input_items (Vec<ItemTypes>): the to do items super structs to be packaged
  ///
  /// # Returns
  /// * (Articles): package struct
  pub fn new(input_articles: Vec<EditingArticle>) -> EditingArticles {
    let mut editing_articles = Vec::new();

    for input_article in input_articles {
      editing_articles.push(input_article);
    }

    return EditingArticles {
      editing_articles: editing_articles
    }
  }
}