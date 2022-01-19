use actix_web::{Responder, HttpRequest};

use super::utils::return_state;
use crate::auth::jwt::JwtToken;

/// This view gets all of the saved to do items that are stored in the state.json file.
///
/// # Arguments
/// * req (HttpRequest): 
///
/// # Returns
/// * (web::Json): all of the stored to do items
pub async fn get(req: HttpRequest) -> impl Responder {
  let token: JwtToken = JwtToken::decode_from_request(req).unwrap();
  return return_state(&token.user_id);
}