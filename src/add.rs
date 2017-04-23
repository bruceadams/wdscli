use clap;
use crossbeam::sync::MsQueue;
use hyper::status::StatusCode;
use info::discovery_service_info;
use select::{select_collection, writable_environment};

use serde_json::to_string;
use std::{cmp, process, thread};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

use wdsapi::common::{ApiError, Credentials};
use wdsapi::document;

struct Context {
    creds: Credentials,
    env_id: String,
    col_id: String,
    retries: u32,
    pace: Duration,
    doc_id: AtomicUsize,
    tick: AtomicUsize,
    too_many_requests: AtomicUsize,
    other_errors: AtomicUsize,
    success: AtomicUsize,
    failure: AtomicUsize,
}

fn final_report(duration: Duration, context: &Context) -> () {
    println!("\nSent {} documents in {} seconds",
             context.success.load(Ordering::Relaxed),
             duration.as_secs());
    println!("Hit {} TooManyRequests (429) responses and {} unexpected errors",
             context.too_many_requests.load(Ordering::Relaxed),
             context.other_errors.load(Ordering::Relaxed));
    println!("{} documents failed",
             context.failure.load(Ordering::Relaxed));
}

fn send_file_with_retry(context: &Context, filename: &str) -> () {
    let mut unexplained_error_count = 0;
    let doc_id = format!("{:011x}",
                         context.doc_id
                                .fetch_add(1, Ordering::Relaxed));
    loop {
        match document::create(&context.creds,
                               &context.env_id,
                               &context.col_id,
                               None,
                               Some(&doc_id),
                               filename) {
            Ok(response) => {
                context.success.fetch_add(1, Ordering::Relaxed);
                println!("{} {}",
                         filename,
                         to_string(&response).unwrap_or_default());
                break;
            }
            Err(e) => {
                if let ApiError::Service(ref se) = e {
                    if se.status_code == StatusCode::TooManyRequests {
                        context.too_many_requests
                               .fetch_add(1, Ordering::Relaxed);
                        // The service says we're going too fast.
                        // Tell the main pace to wait four ticks,
                        // also double sleep here and resend.
                        context.tick.fetch_add(4, Ordering::Relaxed);
                        println!("{} sleep then retry after {}", filename, e);
                        thread::sleep(context.pace
                                             .checked_mul(2)
                                             .unwrap_or(context.pace));
                        continue;
                    }
                }
                context.other_errors.fetch_add(1, Ordering::Relaxed);
                unexplained_error_count += 1;
                if unexplained_error_count <= context.retries {
                    // We will retry, so tell the pace to wait another tick.
                    context.tick.fetch_add(1, Ordering::Relaxed);
                    println!("{} retry after fail to create document {}",
                             filename,
                             e);
                } else {
                    context.failure.fetch_add(1, Ordering::Relaxed);
                    println!("{} give up after fail to create document {}",
                             filename,
                             e);
                    break;
                }
            }
        }
    }
}

fn push_worker(context: &Context, queue: &MsQueue<String>) -> () {
    loop {
        let filename = queue.pop();
        if filename.is_empty() {
            break;
        };
        send_file_with_retry(context, &filename);
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
    let retries: u32 = matches.value_of("retries")
                              .unwrap_or("2")
                              .parse()
                              .expect("Retries must be an integer");
    let threads: u32 = matches.value_of("threads")
                              .unwrap_or("64")
                              .parse()
                              .expect("Threads must be an integer");
    let pace: u64 = matches.value_of("pace")
                           .unwrap_or("500")
                           .parse()
                           .expect("Pace must be an integer");
    let pace = Duration::from_millis(pace);
    let doc_id: usize = match matches.value_of("document-id") {
        Some(id) => {
            usize::from_str_radix(id, 16)
                .expect("Document-id must be a hexadecimal integer")
        }
        None => {
            // Our default starting document id is
            // milliseconds elapsed since the epoch.
            let dur = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::new(0, 0));
            1000 * dur.as_secs() as usize +
            (dur.subsec_nanos() as usize / 1000000)
        }
    };
    let context = Arc::new(Context {
        creds: info.creds.clone(),
        env_id: env_id.clone(),
        col_id: col_id.to_string(),
        retries: retries,
        doc_id: AtomicUsize::new(doc_id),
        pace: pace,
        tick: AtomicUsize::new(0),
        too_many_requests: AtomicUsize::new(0),
        other_errors: AtomicUsize::new(0),
        success: AtomicUsize::new(0),
        failure: AtomicUsize::new(0),
    });
    let queue = Arc::new(MsQueue::new());

    // Fire up a thread pool...
    for _ in 0..threads {
        let worker_item = context.clone();
        let worker_queue = queue.clone();
        thread::spawn(move || push_worker(&worker_item, &worker_queue));
    }

    let base_time = Instant::now();

    // Send work into the thread pool...
    for path in matches.values_of("paths").unwrap() {
        for entry in WalkDir::new(path)
            .sort_by(|a, b| a.cmp(b))
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file()) {
            if let Some(filename) = entry.path().to_str() {
                let filename = filename.to_string();
                context.tick.fetch_add(1, Ordering::Relaxed);
                queue.push(filename);
                while let Some(sleep_duration) =
                    pace.checked_mul(context.tick
                                            .load(Ordering::Relaxed) as
                                     u32)
                        .expect("Ran too long?!")
                        .checked_sub(base_time.elapsed()) {
                    thread::sleep(sleep_duration);
                }
            }
        }
    }

    // Tell my threads to shutdown.
    for _ in 0..threads {
        queue.push(String::new());
    }

    let mut wait_count = 0;
    // Wait for the the threads to complete, but not forever.
    while let Some(item) = queue.try_pop() {
        wait_count += 1;
        queue.push(item);
        thread::sleep(Duration::from_secs(1));
        if wait_count > cmp::max(30, threads) {
            println!("Timeout waiting for worker thread shutdown. Aborting.");
            final_report(base_time.elapsed(), &context);
            process::exit(1);
        }
    }
    final_report(base_time.elapsed(), &context);
}
