use std::collections::HashMap;

use actix_web::{web, get, App, HttpResponse, HttpServer, ResponseError, Result};
use askama::Template;
use thiserror::Error;

#[derive(Template)]
#[template(path="input_form.html")]
struct Index;

#[derive(Template)]
#[template(path="output.html")]
struct UserInformation<'a>{
    height: &'a str,
    weight: &'a str,
}

#[derive(Error, Debug)]
enum MyError {
    #[error ("Failed to render HTML")]
    AsakamaError(#[from] askama::Error),
}
impl ResponseError for MyError{}

#[get("/")]
async fn index(query: web::Query<HashMap<String,String>>) 
-> Result<HttpResponse, MyError> {
    let response_body = if let Some(height) 
    = query.get("height"){
        UserInformation{
            height,
            weight: "50",
        }
        .render()
        .unwrap()
    }else{
        Index.render().unwrap()

    };
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}


#[actix_web::main]
async fn main() ->Result<(), actix_web::Error>{
    HttpServer::new(move || App::new().service(index))
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}