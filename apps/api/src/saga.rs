use actix_web::{get, post, web::{self, Data, Json}, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;

use crate::repository::Repository;
use crate::response::Response;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SagaRequest {
    pub target: String,
    pub target_id: String,
    pub target_ref: Vec<String>,    
    pub timeout: Option<String>,
    pub in_order: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct Saga {
    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct SagaResponse {
    pub saga: Saga,
    pub config: SagaRequest,
}

pub type SagaResponses = Response<SagaResponse>;

pub const APPLICATION_JSON: &str = "application/json";

#[get("/sagas")]
pub async fn list(repo_d: Data<Mutex<Repository>>) -> HttpResponse {
    //riferimento condiviso al singleton
    let repo = repo_d.clone();

    let r = web::block({
        move || {
            SagaResponses {
                results: repo.lock().unwrap().extract()
            }        
        }
    }).await;

    match r {
        Ok(rr) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(rr),
        _ => HttpResponse::InternalServerError().await.unwrap(),
    }
}

#[post("/sagas")]
pub async fn create(req: Json<SagaRequest>, repo_d: Data<Mutex<Repository>>) -> HttpResponse {
    //riferimento condiviso al singleton
    let repo = repo_d.clone();

    let s = web::block({
        move || {
            inner_create(req.0, &mut repo.lock().unwrap()).unwrap()
        }
    }).await;

    match s {
        Ok(ss) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(ss),
        _ => HttpResponse::BadRequest().await.unwrap(),
    }
}

fn inner_create(req: SagaRequest, repo: &mut Repository) -> Result<Saga, Error> {
    Ok(Saga { id: repo.add(req) })
}
