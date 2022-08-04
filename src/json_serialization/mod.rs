pub mod article;
pub mod articles;
pub mod editing_article;
pub mod editing_article_without_article_id;
pub mod editing_articles;
pub mod new_article;

use duplicate::duplicate_item;
use actix_web::{
  Responder, 
  HttpResponse, 
  HttpRequest,
  body::BoxBody
};
use article::Article;
use articles::Articles;
use editing_article::EditingArticle;
use editing_articles::EditingArticles;
use editing_article_without_article_id::EditingArticleWithoutArticleId;

#[duplicate_item(
  name; 
  [Article];
  [Articles]; 
  [EditingArticle];
  [EditingArticles];
  [EditingArticleWithoutArticleId];
)]
impl Responder for name {
  type Body = BoxBody;
  /// This function gets fired when the struct is being returned in an actix view.
  ///
  /// # Arguments
  /// * _req (&HttpRequest): The request belonging to the view.
  ///
  /// # Returns
  /// * HttpResponse<Self::Body>: An OK HTTP response with the serialized struct in the body.
  fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
    let body = serde_json::to_string(&self).unwrap();

    HttpResponse::Ok()
      .content_type("application/json")
      .insert_header(("X-Hdr", "sample"))
      .body(body)
  }
}