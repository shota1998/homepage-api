use actix_web::web;

mod path;
pub mod article;
pub mod editing_article;

/// This function combines the views from other view modules.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None
pub fn routes_factory(app: &mut web::ServiceConfig) {
    article::routes_factory(app);
    editing_article::routes_factory(app);
}
