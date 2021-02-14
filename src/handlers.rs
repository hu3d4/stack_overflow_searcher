use crate::db;
use crate::errors::AppError;
use crate::models::*;
use actix_web::{http::header, web, HttpRequest, HttpResponse, Responder};
use askama::Template;

pub async fn index() -> Result<impl Responder, AppError> {
    let entries = db::show_history()?;
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

pub async fn index_user(req: HttpRequest) -> Result<impl Responder, AppError> {
    let uservalue = req.match_info().get("name").unwrap_or("World");
    let username = uservalue.to_string();
    let html = IndexTemplateUser { username };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

// pub async fn index_user(req: HttpRequest) -> Result<impl Responder, AppError> {
//     let uservalue = req.match_info().get("name").unwrap_or("World");
//     let username = uservalue.to_string();
//     let html = IndexTemplateUser { username };
//     let response_body = html.render()?;
//     Ok(HttpResponse::Ok()
//         .content_type("text/html")
//         .body(response_body))
// }

pub async fn get_history(form: web::Form<GetHistory>) -> Result<impl Responder, AppError> {
    let input = form.input.clone();
    db::get_history(input)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn get_user(form: web::Form<GetUser>) -> Result<impl Responder, AppError> {
    db::get_user(&form)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn delete_history() -> Result<impl Responder, AppError> {
    db::delete_all_history()?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

pub async fn delete_single_history(
    form: web::Form<DeleteHistory>,
) -> Result<impl Responder, AppError> {
    let id = form.id;
    db::delete_one_history(id)?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}
