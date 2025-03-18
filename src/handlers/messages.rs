use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{postgres::Postgres, redis::Redis, schema};

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::messages)]
struct PartialMessage {
    id: i32,
    sender: String,
    is_read: Option<bool>,
    sent_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::messages)]
struct CompleteMessage {
    id: i32,
    sender: String,
    recipient: String,
    content: Option<String>,
    is_read: Option<bool>,
    sent_at: Option<NaiveDateTime>,
}

pub async fn list_messages(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::messages::table
        .select(PartialMessage::as_select())
        .load::<PartialMessage>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(messages)) => {
            return HttpResponse::Ok().json(messages);
        }
        Ok(None) => {
            return HttpResponse::Ok().json(Vec::<PartialMessage>::new());
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_message_by_id(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let result = schema::messages::table
        .select(CompleteMessage::as_select())
        .filter(schema::messages::id.eq(search_id.into_inner()))
        .first::<CompleteMessage>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(message)) => {
            return HttpResponse::Ok().json(message);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_message_by_sender(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_recipient: web::Path<String>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::messages::table
        .select(CompleteMessage::as_select())
        .filter(schema::messages::recipient.eq(&search_recipient.into_inner()))
        .first::<CompleteMessage>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(message)) => {
            return HttpResponse::Ok().json(message);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_message_by_recipient(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_recipient: web::Path<String>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::messages::table
        .select(CompleteMessage::as_select())
        .filter(schema::messages::recipient.eq(&search_recipient.into_inner()))
        .first::<CompleteMessage>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(message)) => {
            return HttpResponse::Ok().json(message);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn delete_message(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    delete_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let count = diesel::delete(schema::messages::table)
        .filter(schema::messages::id.eq(delete_id.into_inner()))
        .returning(CompleteMessage::as_select())
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Ok().finish();
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}
