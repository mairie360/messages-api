use actix_web::web;
use crate::handlers::messages::{list_messages, get_message_by_id, get_message_by_sender, get_message_by_recipient, delete_message};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/messages")
            .route("", web::get().to(list_messages))
            .route("/{id}", web::get().to(get_message_by_id))
            .route("/sender/{sender}", web::get().to(get_message_by_sender))
            .route("/recipient/{recipient}", web::get().to(get_message_by_recipient))
            .route("/{id}", web::delete().to(delete_message))
    );
}