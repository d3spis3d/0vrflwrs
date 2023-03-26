#[macro_use]
extern crate rocket;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions

mod cors;
mod handlers;
mod models;
mod persistance;

use sqlx::postgres::PgPoolOptions;
use std::env;

use cors::*;
use handlers::*;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenvy::dotenv().expect("Unable to setup dotenv");

    let db = env::var("DATABASE_URL").expect("Database URL not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db)
        .await
        .expect("Failed to setup DB pool");

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}
