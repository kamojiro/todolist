use std::sync::Arc;

use actix_web::{App, HttpResponse, HttpServer, ResponseError, get, http::header, post, web};
use askama::Template;
use errors::CustomError;
use tokio;
use serde::Deserialize;
use crate::db::MongoDbClient;
use crate::model::TodoEntry;

mod db;
mod model;
mod errors;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[derive(Deserialize)]
struct AddParams {
    text: String,
}

#[get("/")]
async fn index(mongodb_client: web::Data<Arc<MongoDbClient>>) -> Result<HttpResponse, CustomError> {
    let mut entries = Vec::new();
    let rows = match mongodb_client.get_all_todos().await{
        Ok(x) => x,
        Err(_) => Vec::new(),
    };

    for row in rows {
        entries.push(row);
    }
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[post("/add")]
async fn add_todo(params: web::Form<AddParams>, mongodb_client: web::Data<Arc<MongoDbClient>>) -> Result<HttpResponse, CustomError> {
    mongodb_client.create_todo(&params.text).await?;
    Ok(HttpResponse::SeeOther()
       .append_header((header::LOCATION, "/"))
       .finish())
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let mongodb_uri = "mongodb://localhost:27017".to_string();
    let mongodb_client = Arc::new(MongoDbClient::new(mongodb_uri).await);
    
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(add_todo)
            .app_data(web::Data::new(Arc::clone(&mongodb_client)))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
