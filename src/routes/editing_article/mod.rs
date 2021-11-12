use actix_web::web;
use super::path::Path;
use crate::controller::editing_article;
pub mod get;
mod edit;
mod reflect;
mod reflesh;

pub fn routes_factory(app: &mut web::ServiceConfig) {
  let base_path: Path = Path{prefix: String::from("/editing_article"), backend: true};

  app.route(&base_path.define(String::from("/get_by_id")),
    web::get().to(get::get_by_id));
  app.route(&base_path.define(String::from("/get_by_article_id")),
    web::get().to(get::get_by_article_id));
  app.route(&base_path.define(String::from("/get_all")),
    web::get().to(get::get_all));

  app.route(&base_path.define(String::from("/edit")),
    web::put().to(edit::edit));

  // app.route(&base_path.define(String::from("/reflect")),
  //   web::put().to(reflect::reflect));
  app.route(&base_path.define(String::from("/reflect")),
    web::put().to(editing_article::reflect));

  app.route(&base_path.define(String::from("/reflesh")),
    web::put().to(reflesh::reflesh));
}