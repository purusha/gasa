use reqwest::Client;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tokio::sync::Barrier;
use tokio::task;
//use chrono::Utc;
use std::sync::{Arc, Mutex};

struct Metrics {
    total_requests: usize,
    successful_requests: AtomicUsize,
    failed_requests: AtomicUsize,
    total_response_time: AtomicUsize, // In milliseconds
}

impl Metrics {
    fn new(total_requests: usize) -> Self {
        Metrics {
            total_requests,
            successful_requests: AtomicUsize::new(0),
            failed_requests: AtomicUsize::new(0),
            total_response_time: AtomicUsize::new(0),
        }
    }

    fn record_success(&self, response_time: u64) {
        self.successful_requests.fetch_add(1, Ordering::Relaxed);
        self.total_response_time.fetch_add(response_time as usize, Ordering::Relaxed);
    }

    fn record_failure(&self) {
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
    }

    fn print_summary(&self, duration: f64) {
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
    }
}

#[tokio::main]
async fn main() {
    let url = "http://localhost:8080/sagas";
    let total_requests = 100;
    let parallelism = 10;

    // Use Arc + Mutex for thread safety
    let metrics = Arc::new(Mutex::new(Metrics::new(parallelism))); 

    // Use Arc to share safely
    let client = Arc::new(Client::new()); 

    // Use Arc to share safely
    let barrier = Arc::new(Barrier::new(parallelism));

    let start_time = Instant::now();
    let mut tasks = Vec::new();

    for _ in 0..parallelism {
        let metrics = Arc::clone(&metrics);
        let client = Arc::clone(&client);
        let b = Arc::clone(&barrier);
        let url = url.to_string();

        let task = task::spawn(async move {
            b.wait().await; // Synchronize tasks

            for _ in 0..(total_requests / parallelism) {
                let start = Instant::now();
                
                match client.get(&url).send().await {
                    Ok(response) => {
                        let elapsed = start.elapsed().as_millis() as u64;
                        let metrics = metrics.lock().unwrap();
                        if response.status().is_success() {
                            metrics.record_success(elapsed);
                        } else {
                            metrics.record_failure();
                        }
                    }
                    Err(_) => {
                        let metrics = metrics.lock().unwrap();
                        metrics.record_failure();
                    }
                }
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    let total_duration = start_time.elapsed().as_secs_f64();
    let metrics = metrics.lock().unwrap();
    metrics.print_summary(total_duration);
}