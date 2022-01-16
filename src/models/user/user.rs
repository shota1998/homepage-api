extern crate bcrypt;

use bcrypt::verify;
use diesel::{Identifiable, Queryable};

use crate::schema::users;

#[derive(Queryable, Clone, Identifiable)]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub password: String,
  pub unique_id: String,
}

impl User {
  /// Check password to see if correct.
  ///
  /// # Arguments
  /// * password (String): the password to be checked against the user password
  ///
  /// # Returns
  /// * (bool): true is match, false if not
  pub fn verify(self, password: String) -> bool {
    return verify(password.as_str(), &self.password).unwrap();
  }
}
