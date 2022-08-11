use crate::response::Response;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use model::models::Users;
use model::show_users::show_users;

mod users;
mod response;

const APPLICATION_JSON: &str = "application/json";

pub type ResponseUsers = Response<Users>;

#[get("/")]
pub async fn get_users() -> HttpResponse {
    let users = web::block(move || show_users()).await;

    match users {
        Ok(users) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(ResponseUsers { collection: users }),
        _ => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}
#[post("/echo")]
async fn echo(req_body: String) -> HttpResponse {
    HttpResponse::Created()
        .content_type("application/json")
        .json(req_body)
}

async fn manual_hello() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json("req_body")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_users)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
