use crate::schema::articles;

#[derive(Queryable, Identifiable, Associations)]
// #[belongs_to(User)]
#[table_name="articles"]
pub struct Article {
  pub id    : i32,
  pub title : String,
  pub body  : String
}