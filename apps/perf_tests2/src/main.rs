use reqwest::Client;
use std::time::Instant;
use tokio::sync::Barrier;
use tokio::task;
use std::sync::{Arc, Mutex};

use common_tests::{Metrics, SagaRequest};

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
        let barrier = Arc::clone(&barrier);
        let url = url.to_string();

        let task = task::spawn(async move {
            barrier.wait().await; // Synchronize tasks

            for _ in 0..(total_requests / parallelism) {
                let start = Instant::now();
                let body = SagaRequest::random();

                match client.post(&url).json(&body).send().await {
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
