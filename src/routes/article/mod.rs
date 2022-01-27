use actix_web::web;
use super::path::Path;
mod create;
mod get;
mod edit;
mod delete;
mod utils;

pub fn routes_factory(app: &mut web::ServiceConfig) {
  let base_path: Path = Path{prefix: String::from("/article"), backend: true};
  
  app.route(&base_path.define(String::from("/create")),
    web::post().to(create::create));
  app.route(&base_path.define(String::from("/get")),
    web::get().to(get::get));
  app.route(&base_path.define(String::from("/edit")),
    web::put().to(edit::edit));
  app.route(&base_path.define(String::from("/delete")),
    web::post().to(delete::delete));
}