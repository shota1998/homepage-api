#[macro_use] extern crate diesel;

use actix_web::{App, HttpServer, HttpResponse, http};
use actix_service::Service;
use actix_cors::Cors;
use futures::future::{ok, Either};

use log;
use env_logger;
use std::env;
mod schema;
mod database;
mod models;
mod json_serialization;
mod routes;
mod auth;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Set environmental valuable.
    env::set_var("RUST_LOG", "info");
    // Start logging.
    env_logger::init();
    // Create http server.
    HttpServer::new(|| {
        let cors = Cors::default()
                    .allowed_origin("http://localhost:3000")
                    // .allowed_origin_fn(|origin, _req_head| {
                    //     origin.as_bytes().ends_with(b"localhost")
                    // })
                    .allowed_methods(vec!["GET", "POST", "PUT"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        http::header::CONTENT_TYPE
                    ])
                    .max_age(3600);

        let app = App::new()
            // .wrap(cors)
            .wrap(cors)
            .wrap_fn(|req, srv| {
                // srv => routing
                // req => service request
                
                // Maintain request uri path to be remembered through the process.
                let request_url: String = String::from(*&req.uri().path().clone());
                // If token passed or not.
                let passed: bool;

                // Check token.
                // ??? What is "*&req" ?
                if *&req.path().contains("/item/") {
                    match auth::process_token(&req) {
                        Ok(_token) => {passed = true;},
                        Err(_message) => {passed = false;}
                    };
                } else {
                    passed = true;
                }

                // Take action based on token.
                let end_result = match passed {
                    // Call request.
                    true => {
                        Either::Left(srv.call(req))
                    },
                    // Send body which says failing in process.
                    false => {
                        Either::Right(
                            ok(req.into_response(
                                HttpResponse::Unauthorized()
                                                .finish()
                                                .into_body())
                            )
                        )
                    }
                };

                // Await result to log.
                async move {
                    let result  = end_result.await?;
                    log::info!("{} -> {}", request_url, &result.status());
                    Ok(result)
                }
            })
            .configure(routes::routes_factory);
        return app
    })
    // .bind("127.0.0.1:8000")?
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
