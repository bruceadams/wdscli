use clap;
use crossbeam::sync::MsQueue;
use hyper::status::StatusCode;
use info::discovery_service_info;
use select::{select_collection, writable_environment};

use serde_json::to_string;
use std::{thread, time};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use walkdir::WalkDir;

use wdsapi::common::{ApiError, Credentials};
use wdsapi::document;

struct WorkItem {
    creds: Credentials,
    env_id: String,
    col_id: String,
    sleep_duration: time::Duration,
    extra_sleep_count: AtomicUsize,
}

fn send_file_with_retry(work_item: &WorkItem, filename: &str) -> () {
    let mut unexplained_error_count = 0;
    loop {
        match document::create(&work_item.creds,
                               &work_item.env_id,
                               &work_item.col_id,
                               None,
                               None,
                               filename) {
            Ok(response) => {
                println!("{} {}",
                         filename,
                         to_string(&response).unwrap_or_default());
                break;
            }
            Err(e) => {
                if let ApiError::Service(ref se) = e {
                    if se.status_code == StatusCode::TooManyRequests {
                        // The service says we're going too fast.
                        // Tell the pace thread to skip two beats,
                        // also sleep here and resend.
                        work_item.extra_sleep_count
                                 .fetch_add(2, Ordering::Relaxed);
                        println!("{} sleep then retry after {}",
                                 filename,
                                 StatusCode::TooManyRequests);
                        thread::sleep(work_item.sleep_duration);
                        continue;
                    }
                }
                unexplained_error_count += 1;
                if unexplained_error_count < 3 {
                    // We will retry, so tell the pace thread to skip a beat.
                    work_item.extra_sleep_count.fetch_add(1, Ordering::Relaxed);
                    println!("{} retry after fail to create document {}",
                             filename,
                             e);
                } else {
                    println!("{} give up after fail to create document {}",
                             filename,
                             e);
                    break;
                }
            }
        }
    }
}

fn push_worker(work_item: &WorkItem, queue: &MsQueue<String>) -> () {
    loop {
        let filename = queue.pop();
        if filename.is_empty() {
            break;
        };
        send_file_with_retry(work_item, &filename);
    }
}

pub fn add_document(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(creds);
    let env_info = writable_environment(&info);
    let collection = select_collection(&env_info, matches);
    let env_id = env_info.environment_id;
    let col_id = collection["collection_id"]
        .as_str()
        .expect("Internal error: missing collection_id");
    let pace: u64 = matches.value_of("pace")
                           .unwrap_or("500")
                           .parse()
                           .expect("Pace must be an integer");
    let sleep_duration = time::Duration::from_millis(pace);
    let work_item = Arc::new(WorkItem {
        creds: info.creds.clone(),
        env_id: env_id.clone(),
        col_id: col_id.to_string(),
        sleep_duration: sleep_duration,
        extra_sleep_count: AtomicUsize::new(0),
    });
    let queue = Arc::new(MsQueue::new());

    let thread_count = 64;
    // Fire up a thread pool...
    for _ in 0..thread_count {
        let worker_item = work_item.clone();
        let worker_queue = queue.clone();
        thread::spawn(move || push_worker(&worker_item, &worker_queue));
    }

    // Send work into the thread pool...
    for path in matches.values_of("paths").unwrap() {
        for entry in WalkDir::new(path)
            .sort_by(|a, b| a.cmp(b))
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file()) {
            if let Some(filename) = entry.path().to_str() {
                let filename = filename.to_string();
                queue.push(filename);
                thread::sleep(sleep_duration);
                // This is safe only because this is the only place where
                // `extra_sleep_count` is decremented.
                while work_item.extra_sleep_count.load(Ordering::Relaxed) > 0 {
                    work_item.extra_sleep_count.fetch_sub(1, Ordering::Relaxed);
                    thread::sleep(sleep_duration);
                }
            }
        }
    }

    // Tell my threads to shutdown.
    for _ in 0..thread_count {
        queue.push(String::new());
    }

    while let Some(item) = queue.try_pop() {
        queue.push(item);
        thread::sleep(time::Duration::from_secs(1));
    }
}
