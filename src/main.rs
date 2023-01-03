#![deny(
    non_ascii_idents,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_must_use,
    clippy::unwrap_used
)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use casbin::{CoreApi, Enforcer};
use paperclip::actix::web::{get, post};
use paperclip::actix::{web, OpenApiExt};
use paperclip::v2::models::{DefaultApiRaw, Info};
use sqlx::postgres::PgPoolOptions;

use service::account::AccountService;

use crate::service::casbin::CasbinService;
use crate::service::token::TokenService;

mod error;
mod handler;
mod model;
mod service;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    configure_logger();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let workers = dotenv::var("WORKERS")
        .expect("WORKERS must be set")
        .parse()
        .expect("Invalid WORKERS");
    let access_token_lifetime_ms: i64 = dotenv::var("ACCESS_TOKEN_LIFETIME_MS")
        .expect("ACCESS_TOKEN_LIFETIME_MS must be set")
        .parse()
        .expect("Invalid ACCESS_TOKEN_LIFETIME_MS");
    let refresh_token_lifetime_ms: i64 = dotenv::var("REFRESH_TOKEN_LIFETIME_MS")
        .expect("REFRESH_TOKEN_LIFETIME_MS must be set")
        .parse()
        .expect("Invalid REFRESH_TOKEN_LIFETIME_MS");
    let port = dotenv::var("PORT").expect("PORT must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Unable to connect to DB");

    let account_service = AccountService {
        pool: pool.clone(),
        secret: secret.clone(),
    };
    let token_service = TokenService {
        secret,
        access_token_lifetime_ms,
        refresh_token_lifetime_ms,
    };
    let enforcer = Enforcer::new("casbin/model.conf", "casbin/policy.csv")
        .await
        .expect("Failure to load enforcer");

    let casbin_service = CasbinService { enforcer };
    let casbin_service = Arc::new(casbin_service);

    log::info!("Server started");

    HttpServer::new(move || {
        let spec = DefaultApiRaw {
            info: Info {
                version: "0.1".into(),
                title: "Auth".into(),
                ..Default::default()
            },
            ..Default::default()
        };
        let cors = Cors::permissive();
        App::new()
            .wrap_api_with_spec(spec)
            .wrap(Logger::default())
            .wrap(cors)
            .service(
                web::scope("/accounts")
                    .route("/permission", get().to(handler::policy::permissions))
                    .route("/session", post().to(handler::session::create)),
            )
            .app_data(web::Data::new(account_service.clone()))
            .app_data(web::Data::new(token_service.clone()))
            .app_data(web::Data::new(casbin_service.clone()))
            .wrap(Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
            ))
            .with_json_spec_at("/swagger-spec")
            .with_swagger_ui_at("/swagger-ui")
            .build()
    })
    .bind(format!("127.0.0.1:{}", port))
    .expect("Failed to run server")
    .workers(workers)
    .run()
    .await
}

fn configure_logger() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout())
        .chain(fern::log_file("log.log").expect("Failure create log file"))
        .apply()
        .expect("Failure configure logger");
}
