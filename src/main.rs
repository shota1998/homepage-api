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

// #[actix_web::main]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Create http server.
    HttpServer::new(|| {
        let cors = Cors::default()
                    .allowed_origin(&env::var("ALLOWED_ORIGIN_1").expect("ALLOWED_ORIGIN_1 must be set."))
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
                // Maintain request uri path to be remembered through the process.
                let request_url: String = String::from(*&request.uri().path().clone());
                // If token passed or not.
                let passed: bool;

                // Check token.
                if *&request.path().contains("/item/") {
                    match auth::process_token(&request) {
                        Ok(_token) => {passed = true;},
                        Err(_message) => {passed = false;}
                    };
                }
                else {
                    passed = true;
                }

                // Take action based on token.
                let end_result = match passed {
                    // Call request.
                    true => {
                        Either::Left(service.call(request))
                    },
                    // Send body which says failing in process.
                    false => {
                        let resp = HttpResponse::Unauthorized().finish();
                        Either::Right(ok(request.into_response(resp).map_into_right_body()))
                    }
                };

                // Await result to be loged.
                async move {
                    let result = end_result.await?;
                    log::info!("{} -> {}", request_url, &result.status());
                    Ok(result)
                }
            })
            .configure(routes::routes_factory);
        return app
    })
    // .bind("127.0.0.1:8000")?
    .bind("0.0.0.0:8001")?
    .run()
    .await
}