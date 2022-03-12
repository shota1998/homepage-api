use actix_web::web;
use super::path::Path;
mod create;
mod get;
mod edit;
mod delete;

pub fn routes_factory(app: &mut web::ServiceConfig) {
  let base_path: Path = Path{prefix: String::from("/article"), backend: true};
  
  app.route(&base_path.define(String::from("/create")),
    web::post().to(create::create));
  app.route(&base_path.define(String::from("/get_article_by_id")),
    web::get().to(get::get_article_by_id));
  app.route(&base_path.define(String::from("/get_all_articles")),
    web::get().to(get::get_all_articles));
  app.route(&base_path.define(String::from("/get_editing_article_by_id")),
    web::get().to(get::get_editing_article_by_id));
  app.route(&base_path.define(String::from("/get_editing_article_by_article_id")),
    web::get().to(get::get_editing_article_by_article_id));
  app.route(&base_path.define(String::from("/get_all_editing_articles")),
    web::get().to(get::get_all_editing_articles));
  app.route(&base_path.define(String::from("/edit_article")),
    web::put().to(edit::edit_article));
  app.route(&base_path.define(String::from("/edit_editing_article")),
    web::put().to(edit::edit_editing_article));
  app.route(&base_path.define(String::from("/reflesh_editing_article")),
    web::put().to(edit::reflesh_editing_article));
  app.route(&base_path.define(String::from("/delete")),
    web::post().to(delete::delete));
}