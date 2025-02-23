use std::sync::atomic::{AtomicUsize, Ordering};

mod randomizer;
use csv::Writer;
use serde::Serialize;

use crate::randomizer::*;

use chrono::{Duration as ChronoDuration, Local, NaiveTime};

pub struct Metrics {
    successful_requests: Vec<u64>,          // In milliseconds
    failed_requests: Vec<u64>,              // count of 
    total_response_time: AtomicUsize,       // In milliseconds
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            successful_requests: vec![],
            failed_requests: vec![],
            total_response_time: AtomicUsize::new(0),
        }
    }

    pub fn record_success(&mut self, response_time: u64) {
        self.successful_requests.push(response_time);
        self.total_response_time.fetch_add(response_time as usize, Ordering::Relaxed);
    }

    pub fn record_failure(&mut self, response_time: u64) {
        self.failed_requests.push(response_time);
    }

    pub fn print_summary(&self, duration: f64) {
        let successful = self.successful_requests.iter().count();
        let failed = self.failed_requests.iter().count();
        let total_time = self.total_response_time.load(Ordering::Relaxed);

        let avg_response_time = if successful > 0 {
            total_time as f64 / successful as f64
        } else {
            0.0
        };

        println!("Summary of Load Test:");
        println!("-------------------------------------");
        println!("Total Requests: {}", successful + failed);
        println!("Successful Requests: {}", successful);
        println!("Failed Requests: {}", failed);
        println!("Total Duration: {:.2} seconds", duration);
        println!("Average Response Time: {:.2} ms", avg_response_time);
        println!("Throughput: {:.2} requests/second", successful as f64 / duration);
        println!("-------------------------------------");
        println!("Response times (ms): {:?} and {:?}", self.successful_requests, self.failed_requests); 

        /*
            Richieste totali: numero di richieste inviate.
            Richieste di successo: conteggio di risposte con codice HTTP 2xx.
            Richieste fallite: conteggio di errori o codici HTTP non 2xx.
            Tempo medio di risposta: calcolato sommando i tempi delle risposte riuscite.
            Throughput: richieste per secondo.
        */

        self.print_csv();
    }

    fn print_csv(&self) {
        let now = Local::now();
    
        // Crea il nome del file nel formato "avvio_GG-MM-AAAA_OO-MM-SS.txt"
        let filename = format!("bi/data/gasa-{}.csv", now.format("%d-%m-%Y_%H-%M-%S"));        
        let mut wtr = Writer::from_path(filename).unwrap();

        let from = NaiveTime::from_hms_opt(00, 00, 00).unwrap();
        let mut looop: i64 = 0;

        //intestazione
        let _ = wtr.write_record(&["timestamp", "response_time", "status"]);

        //200
        for i in &self.successful_requests {
            looop += 1;
            let current_time = from + ChronoDuration::seconds(looop);
            let _ = wtr.write_record(
                &[ String::from("2025-02-22 ") + &current_time.format("%H:%M:%S").to_string(), i.to_string(), String::from("200") ]
            );
        }

        //500
        for i in &self.failed_requests {
            looop += 1;
            let current_time = from + ChronoDuration::seconds(looop);
            let _ = wtr.write_record(
                &[ String::from("2025-02-22 ") + &current_time.format("%H:%M:%S").to_string(), i.to_string(), String::from("500") ]
            );
        }
                
        let _ = wtr.flush();        
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
