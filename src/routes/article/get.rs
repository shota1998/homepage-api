use actix_web::{Responder, Error, HttpResponse, HttpRequest};
use serde::Serialize;
use futures::future::{ready, Ready};
use log;
// use super::utils::return_state;
use crate::auth::jwt::JwtToken;

#[derive(Serialize)]
pub struct Article {
  pub title:  String,
  pub author: String
}

impl Article {
  pub fn new(title: String, author: String) -> Article {
    return Article {
      title:  title, 
      author: author
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

/// This view gets all of the saved to do items that are stored in the state.json file.
///
/// # Arguments
/// * req (HttpRequest): 
///
/// # Returns
/// * (web::Json): all of the stored to do items
pub async fn get(req: HttpRequest) -> impl Responder {
  // let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

  return Article{
    title:  String::from("sample_title"), 
    author: String::from("sample_author")
  };
}