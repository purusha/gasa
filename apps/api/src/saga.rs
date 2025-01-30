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

/// list 50 last tweets `/tweets`
#[get("/sagas")]
pub async fn list(repo_d: Data<Mutex<Repository>>) -> HttpResponse {
    //riferimento condiviso al singleton
    let repo = repo_d.clone();

    let extract = repo.lock().unwrap().extract();

    let responses = SagaResponses {
        results: extract.iter()
            .map(|t| {
                SagaResponse {
                    saga: Saga {id: t.0},
                    config: t.1.clone()
                }
            })
            .collect()
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(responses)

}

#[post("/sagas")]
pub async fn create(req: Json<SagaRequest>, repo_d: Data<Mutex<Repository>>) -> HttpResponse {
    let s = web::block({

        //riferimento condiviso al singleton
        let repo = repo_d.clone();

        move || {
            // accedi al Repository protetto dal Mutex
            let mut repo = repo.lock().unwrap();

            //no ownership ... ma call with ref (also mutable)
            inner_create(req.0, &mut repo).unwrap()
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
