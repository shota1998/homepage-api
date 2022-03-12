use serde::Deserialize;
use serde::Serialize;
use futures::future::{ready, Ready};
use actix_web::{Responder, Error, HttpResponse, HttpRequest};

#[derive(Deserialize, Serialize, Clone)]
pub struct EditingArticleWithoutArticleId {
  pub id         : i32,
  pub title      : String,
  pub body       : String
}

impl EditingArticleWithoutArticleId {
  /// Constructs the EditingArticleWithoutArticleId struct.
  ///
  /// # Arguments
  /// * id (i32): 
  /// * title (String):
  /// * body (String):
  ///
  /// # Returns
  /// * (EditingArticleWithoutArticleId): 
  pub fn new(id: i32, title: String, body: String) -> EditingArticleWithoutArticleId {
    return EditingArticleWithoutArticleId {
      id         : id,
      title      : title,
      body       : body
    }
  }
}

impl Responder for EditingArticleWithoutArticleId {
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