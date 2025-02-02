use crate::saga::{self, Saga, SagaResponse};

use dashmap::DashMap;
use saga::SagaRequest;
use uuid::Uuid;

pub struct Repository {
    data: DashMap<Uuid, SagaRequest>,
}

impl Repository {
    pub fn new() -> Repository {
        Repository {
            data: DashMap::new()
        }
    }

    //no ownership ... ma call with ref (also mutable)
    pub fn add(&mut self, sr: SagaRequest) -> Uuid {
        let key: Uuid = Uuid::new_v4();
        self.data.insert(key, sr);

        key
    }

    pub fn extract(&self) -> Vec<SagaResponse> {
        self.data
            .iter()
            .map(|entry| {
                SagaResponse {
                    saga: Saga {id: entry.key().clone()},
                    config: entry.value().clone()
                }
            })
            .collect()
    }
    
}
