use std::sync::atomic::{AtomicUsize, Ordering};

mod randomizer;
use serde::Serialize;

use crate::randomizer::*;
pub struct Metrics {
    total_requests: usize,
    successful_requests: AtomicUsize,
    failed_requests: AtomicUsize,
    total_response_time: AtomicUsize, // In milliseconds
}

impl Metrics {
    pub fn new(total_requests: usize) -> Self {
        Metrics {
            total_requests,
            successful_requests: AtomicUsize::new(0),
            failed_requests: AtomicUsize::new(0),
            total_response_time: AtomicUsize::new(0),
        }
    }

    pub fn record_success(&self, response_time: u64) {
        self.successful_requests.fetch_add(1, Ordering::Relaxed);
        self.total_response_time.fetch_add(response_time as usize, Ordering::Relaxed);
    }

    pub fn record_failure(&self) {
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn print_summary(&self, duration: f64) {
        let successful_requests = self.successful_requests.load(Ordering::Relaxed);
        let failed_requests = self.failed_requests.load(Ordering::Relaxed);
        let total_response_time = self.total_response_time.load(Ordering::Relaxed);

        let avg_response_time = if successful_requests > 0 {
            total_response_time as f64 / successful_requests as f64
        } else {
            0.0
        };

        println!("Summary of Load Test:");
        println!("Total Requests: {}", self.total_requests);
        println!("Successful Requests: {}", successful_requests);
        println!("Failed Requests: {}", failed_requests);
        println!("Total Duration: {:.2} seconds", duration);
        println!("Average Response Time: {:.2} ms", avg_response_time);
        println!("Throughput: {:.2} requests/second", successful_requests as f64 / duration);

        /*
            Richieste totali: numero di richieste inviate.
            Richieste di successo: conteggio di risposte con codice HTTP 2xx.
            Richieste fallite: conteggio di errori o codici HTTP non 2xx.
            Tempo medio di risposta: calcolato sommando i tempi delle risposte riuscite.
            Throughput: richieste per secondo.
         */

    }
}

/*
    TODO: use SagaRequest struct define in api project
 */
#[derive(Debug, Serialize)]
pub struct SagaRequest {
    pub target: String,
    pub target_id: String,
    pub target_ref: Vec<String>,    
}

impl SagaRequest {

    /* 
        TODO: when api act validation on request object, random data should not be valid
    */
    pub fn random() -> SagaRequest {
        SagaRequest { 
            target: generate_random_string(10), 
            target_id: random_u64().to_string(), 
            target_ref: vec![ generate_random_string(10), generate_random_string(10), generate_random_string(10) ] 
        }
    }
}
