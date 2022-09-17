#[macro_use] extern crate diesel;

use actix_web::{
    App,
    HttpServer,
    HttpResponse,
    http
};
use actix_web::dev::Service;
use actix_cors::Cors;
use futures::future::{ok, Either};
use std::env;
use dotenv::dotenv;

mod schema;
mod database;
mod models;
mod json_serialization;
mod routes;
mod auth;
mod sdk;
mod file;
mod my_regex;
mod controller;
mod logic;
mod constants;
mod test_utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
                    .allowed_origin(&env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN must be set."))
                    .allowed_methods(vec!["GET", "POST", "PUT"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        http::header::CONTENT_TYPE
                    ])
                    .max_age(3600);

        let app = App::new()
            .wrap(cors)
            .wrap_fn(|request, service| {

                let request_url: String = String::from(*&request.uri().path().clone());
                let passed: bool;

                
                if *&request.path().contains("/item/") {
                    match auth::process_token(&request) {
                        Ok(_token) => {passed = true;},
                        Err(_message) => {passed = false;}
                    };
                }
                else {
                    passed = true;
                }

                
                let end_result = match passed {
                    true => {
                        Either::Left(service.call(request))
                    },
                    false => {
                        let resp = HttpResponse::Unauthorized().finish();
                        Either::Right(ok(request.into_response(resp).map_into_right_body()))
                    }
                };


                async move {
                    let result = end_result.await?;
                    log::info!("{} -> {}", request_url, &result.status());
                    Ok(result)
                }
            })
            .configure(routes::routes_factory);
        return app
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}