use crate::schema::tmp_articles;

#[derive(Queryable, Identifiable, Associations)]
#[table_name="tmp_articles"]
pub struct TmpArticle {
  pub title : String,
  pub body  : String
}