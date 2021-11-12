use actix_web::dev::ServiceRequest;

/// Processes the token to see if the correct token is in the header.
/// 
/// # Paramerers
/// * request (&ServiceRequest): the request passed through the view function 
/// 
/// # Returns
/// * (Result<String, &'templates str>): processed token if successful, error message if not.
pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
  match extract_header_token(request) {
    Ok(token) => check_password(token),
    Err(message) => Err(message)
  }
}

/// Checks to see if the token matches. 
/// 
/// # Parameters
/// * password (String): password to be checked.
/// 
/// # Returns
/// * (Result<String, &'templates str>): password if correct, error message if not.
fn check_password(password: String) -> Result<String, &'static str> {
  if password == "token" {
    return Ok(password)
  }

  return Err("token not authorised")
}

/// Exrracts thr header from the request.
/// 
/// # Parameters
/// * request (&ServiceRequest): the request passed through the view function.
/// 
/// # Returns
/// * (Result<String, &'templates str>): processed token if successful, errir message if not.
fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
  match request.headers().get("user-token") {
    Some(token) => {
      match token.to_str() {
        Ok(processed_password) => Ok(String::from(processed_password)),
        Err(_processed_password) => Err("there was an error processing token")
      }
    },
    None => Err("there is no token")
  }
}