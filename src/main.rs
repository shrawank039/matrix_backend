#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use serde::Serialize;

mod schema;
mod models;

use self::models::User;

#[derive(Serialize)]
struct UserResponse {
    id: i32,
    name: String,
    email: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/users").route(web::get().to(get_users)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

async fn get_users() -> impl Responder {
    use self::schema::users::dsl::*;
    
    let connection = &mut establish_connection();
    let results = users.load::<User>(connection).expect("Error loading users");

    let user_response: Vec<UserResponse> = results.into_iter().map(|user| {
        UserResponse { 
            id: user.id, 
            name: user.name, 
            email: user.email 
        }
    }).collect();

    HttpResponse::Ok().json(user_response)
}
