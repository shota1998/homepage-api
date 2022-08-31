use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

/// Initialize database connection.
/// # Returns
/// * (PgConnection)
pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = 
      if cfg!(test) {
        env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set.")
      } 
      else {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set.")
      };

  log::info!("DB URL : {}", database_url);

  PgConnection::establish(&database_url)
    .unwrap_or_else(|msg| panic!("URL: {} : {}", database_url, msg))
}
