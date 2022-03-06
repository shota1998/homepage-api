use crate::schema::articles;
use crate::schema::tmp_articles;

#[derive(Queryable, Identifiable, Associations)]
#[table_name="articles"]
pub struct Article {
  pub id    : i32,
  pub title : String,
  pub body  : String
}

#[derive(Queryable, Identifiable, Associations)]
#[table_name="tmp_articles"]
pub struct TmpArticle {
  pub id    : i32,
  pub title : String,
  pub body  : String
}