use crate::{DATE_FORMAT, State};
use actix_web::{
    HttpResponse, Responder, Result, error::ErrorInternalServerError, get,
    http::header::ContentType, post, web,
};
use serde::Deserialize;
use time::Date;

#[derive(Deserialize)]
pub struct JournalAppend {
    content: String,
    depth: usize,
}

// https://kota.nz/blank_favicon.html
#[get("/favicon.ico")]
pub async fn favicon() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::png())
        .body(web::Bytes::new())
}

#[post("/journal/{day}")]
pub async fn journal_append_block(
    path: web::Path<String>,
    request: web::Json<JournalAppend>,
    state: web::Data<State>,
) -> Result<impl Responder> {
    let day = path.into_inner();

    let mut journal = if day == "today" {
        state.graph.today().map_err(ErrorInternalServerError)?
    } else {
        let day = Date::parse(&day, &DATE_FORMAT).map_err(ErrorInternalServerError)?;

        state.graph.journal(day).map_err(ErrorInternalServerError)?
    };

    journal
        .append_block(&request.content, request.depth)
        .map_err(ErrorInternalServerError)?;
    state
        .graph
        .save(&mut journal)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok())
}
