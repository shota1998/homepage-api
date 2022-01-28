use serde::Deserialize;
use serde::Serialize;
use futures::future::{ready, Ready};
use actix_web::{Responder, Error, HttpResponse, HttpRequest};

#[derive(Deserialize, Serialize, Clone)]
pub struct Article {
  pub id: i32,
  pub title: String,
  // pub date: String,
  // pub genre: String
  pub body: String
}

impl Article {
  /// This function constructs the Articles struct.
  ///
  /// # Arguments
  /// * input_items (Vec<ItemTypes>): the to do items super structs to be packaged
  ///
  /// # Returns
  /// * (Articles): package struct
  pub fn new(id: i32, title: String, body: String) -> Article {
    return Article {
      id    : id,
      title : title,
      body  : body
    }
  }
}

impl Responder for Article {
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