use serde::Deserialize;
use serde::Serialize;
use futures::future::{ready, Ready};
use actix_web::{Responder, Error, HttpResponse, HttpRequest};

#[derive(Deserialize, Serialize, Clone)]
pub struct EditingArticle {
  pub id         : i32,
  pub article_id : i32,
  pub title      : String,
  pub body       : String
}

impl EditingArticle {
  /// Constructs the EditingArticle struct.
  ///
  /// # Arguments
  /// * id (i32): 
  /// * article_id (i32):
  /// * title (String):
  /// * body (String):
  ///
  /// # Returns
  /// * (EditingArticle): 
  pub fn new(id: i32, article_id: i32, title: String, body: String) -> EditingArticle {
    return EditingArticle {
      id         : id,
      article_id : article_id,
      title      : title,
      body       : body
    }
  }
}

impl Responder for EditingArticle {
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