use actix_web::{App, HttpServer};
mod json_serialization;
mod processes;
mod state;
mod to_do;
mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(views::views_factory))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
