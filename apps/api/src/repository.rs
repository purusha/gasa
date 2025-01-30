use crate::saga;

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

    pub fn extract(&self) -> Vec<(Uuid, SagaRequest)> {
        self.data
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect()
    }
    
}
