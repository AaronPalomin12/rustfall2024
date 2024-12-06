// Aaron Palomin
// This function is to import the required modules.
// `mpsc` is used for sending messages between threads and allows a worker to return results to one main thread.
// Then, `thread` is the module which is needed for spawning multiple worker threads which will do the concurrent processing.
// Once imported, `Duration` is used to set timeout durations which would be used for checking websites.
// It brings in `check_website` from the `http_client` module so it can actually perform checks on a website.
// It imports the `WebsiteStatus` struct from the `website_status` module for encapsulating results
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::http_client::check_website;
use crate::website_status::WebsiteStatus;

// This function creates multiple running threads to check the web sites simultaneously. It takes a list of URLs, a timeout period, the number of retries, the number of workers, and an mpsc channel sender.
pub fn start_workers(urls: Vec<String>, timeout: Duration, retries: u8, worker_count: usize, tx: mpsc::Sender<WebsiteStatus>) 
{
    // The piece size is calculated for equally dividing the URLs among all the worker threads. 
    // This ensures that every worker has approximately the same amount of URLs to process.
    let chunk_size = (urls.len() + worker_count - 1) / worker_count; 

   // Disconnecting the URLs into segments and creating a worker thread for every segment.
    for chunk in urls.chunks(chunk_size) 
    {
        let tx = tx.clone(); 
        let timeout = timeout.clone();
        let retries = retries;
        let chunk = chunk.to_vec(); 

        // Initiating a worker thread for processing URLs from the present chunk.
        thread::spawn(move || {
            for url in chunk 
            {
                // It's now executing the `check_website` method for each URL in the list.
                let status = check_website(&url, timeout, retries); 
                // Sending the result back to the main thread through the channel: the `WebsiteStatus` struct. 
                // If it fails to send, we throw an error using `expect`.
                tx.send(status).expect("Failed to send status"); 
            }
        });
    }
}
