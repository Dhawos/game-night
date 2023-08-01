use actix_web::{Responder, HttpResponse, http::header::ContentType, body::BoxBody};

use super::types::BoardGame;

impl Responder for BoardGame {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}