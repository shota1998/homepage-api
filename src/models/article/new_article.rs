use crate::schema::articles;

#[derive(Insertable)]
#[table_name="articles"]
pub struct Article {
  pub id    : i32,
  pub title : String,
  pub body  : String
}