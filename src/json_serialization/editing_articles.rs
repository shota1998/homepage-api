use std::vec::Vec;
use serde::Serialize;
use futures::future::{ready, Ready};
use actix_web::{Responder, Error, HttpResponse, HttpRequest};

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

impl Responder for EditingArticles {
  type Error  = Error;
  type Future = Ready<Result<HttpResponse, Error>>;
  /// This function gets fired when the struct is being returned in an actix view.
  ///
  /// # Arguments
  /// * _req (&HttpRequest): The request belonging to the view.
  ///
  /// # Returns
  /// * (Self::Future): An OK HTTP response with the serialized struct in the body.
  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self).unwrap();
    ready(Ok(HttpResponse::Ok()
      .content_type("application/json")
      .body(body)))
  }
}