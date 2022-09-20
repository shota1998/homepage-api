use actix_web::web;
use super::path::Path;
pub mod create;
mod get;
mod delete;

pub fn routes_factory(app: &mut web::ServiceConfig) {
  let base_path: Path = Path{prefix: String::from("/article"), backend: true};
  
  app.route(&base_path.define(String::from("/create")),
    web::post().to(create::create));

  app.route(&base_path.define(String::from("/get_by_id")),
    web::get().to(get::get_by_id));
  app.route(&base_path.define(String::from("/get_all")),
    web::get().to(get::get_all));
    
  app.route(&base_path.define(String::from("/delete")),
    web::post().to(delete::delete));
}