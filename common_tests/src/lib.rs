use std::sync::atomic::{AtomicUsize, Ordering};

mod randomizer;
use csv::Writer;
use serde::Serialize;

use crate::randomizer::*;

use chrono::{Duration as ChronoDuration, Local, NaiveTime};

use std::fs;
use std::path::Path;
pub struct Metrics {
    successful_requests: Vec<u64>,          // In milliseconds
    failed_requests: Vec<u64>,              // count of 
    total_response_time: AtomicUsize,       // In milliseconds
    app_name: String
}

impl Metrics {
    pub fn new(app_name: String) -> Self {
        Metrics {
            successful_requests: vec![],
            failed_requests: vec![],
            total_response_time: AtomicUsize::new(0),
            app_name
        }
    }

    pub fn record_success(&mut self, response_time: u64) {
        self.successful_requests.push(response_time);
        self.total_response_time.fetch_add(response_time as usize, Ordering::Relaxed);
    }

    pub fn record_failure(&mut self, response_time: u64) {
        self.failed_requests.push(response_time);
        self.total_response_time.fetch_add(response_time as usize, Ordering::Relaxed);
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
        let parent_dir = Path::new("bi/data");
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).unwrap();
            println!("Directory creata.");
        }

        let now = Local::now();
        let filename = format!("bi/data/{}-{}.csv", self.app_name, now.format("%d-%m-%Y_%H-%M-%S"));        
        let mut wtr = Writer::from_path(filename).unwrap();

        let from = NaiveTime::from_hms_opt(00, 00, 00).unwrap();        

        //intestazione
        wtr.write_record(&["timestamp", "response_time", "status"]).unwrap();

        //data
        let mut seconds: i64 = 0;
        for num in self.successful_requests.iter().map(|x| (x, 200))
            .chain(self.failed_requests.iter().map(|x| (x, 500))) {
                seconds += 1;
                let time = (from + ChronoDuration::seconds(seconds)).format("%H:%M:%S").to_string();
                let today = now.format("%Y-%m-%d ").to_string();

                wtr.write_record(&[ today + &time, num.0.to_string(), num.1.to_string() ]).unwrap();
            }
                
        wtr.flush().unwrap();
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
