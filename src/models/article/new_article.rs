use crate::schema::articles;

#[derive(Insertable)]
#[table_name="articles"]
pub struct NewArticle {
  pub title : String,
  pub body  : String
}

// pub struct NewPost<'a> {
//   pub title: &'a str,
//   pub body: &'a str,
// }

impl NewArticle {

  /// Creates a new instance of the NewUser struct.
  ///
  /// # Arguments
  /// * name (String): The name of the user
  /// * username (String): The username of the user
  /// * email (String): The email associated  with the user
  /// * password (String): The password for the user
  ///
  /// # Returns
  /// (NewUser):
  pub fn new(title: String, body: String) -> NewArticle {
    return NewArticle {title, body};
  }
}