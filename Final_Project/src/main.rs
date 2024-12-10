// Aaron Palomin
// I have imported several modules here to accomplish different functionalities:
// `std::env`: this is to read the environment variables or command-line arguments.
// `std::time`: to measure time duration and check the time elapsed.
// `std::sync`: It facilitates thread-safe shared data with the use of `Arc` and `Mutex`.
// `std::thread`: to allow concurrent execution with threads.
// `chrono::Utc`: For getting the current timestamp
// `ureq`: For making HTTP calls.

use std::env;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;
use chrono::Utc;
use ureq;

// This imports the `website_status` package, which defines the struct `WebsiteStatus`.
mod website_status;

fn main() 
{
    // In this part, I set a specific timeout duration for webpage checks using the `get_timeout_duration` function.
    let timeout_duration = get_timeout_duration();
    // Here is the list of site URLs I would like to have them confirmed. I have included a mixed variety of sites for testing purposes.
    let urls = vec![
        "http://www.apple.com", "http://www.microsoft.com", "http://www.xbox.com", "http://www.bestbuy.com", 
        "http://www.target.com", "http://www.walmart.com", "http://www.gamestop.com", "http://www.nfl.com", 
        "http://www.playstation.com", "http://www.utrgv.com", "http://www.paramountplus.com", "http://www.hulu.com", 
        "http://www.disneyplus.com", "http://www.peacocktv.com", "http://www.max.com", "http://www.pff.com", 
        "http://www.cinemark.com", "http://www.espn.com", "http://www.nintendo.com", "http://www.lg.com", 
        "http://www.spacex.com", "http://www.toyota.com", "http://abc.com", "http://www.foxnews.com", 
        "http://www.sonicdrivein.com", "http://www.nbc.com", "http://www.rockstargames.com", "http://www.ubisoft.com", 
        "http://www.nba2k.com", "http://www.dominos.com", "http://www.nike.com", "http://www.tacobell.com", 
        "http://www.underarmour.com", "http://www.footlocker.com", "http://www.samsung.com", "http://www.dairyqueen.com", 
        "http://www.marvel.com", "http://www.dc.com", "http://www.android.com", "http://www.jackinthebox.com", 
        "http://www.nba.com", "http://www.texasroadhouse.com", "http://www.samsclub.com", "http://www.costco.com", 
        "http://www.olivegarden.com", "http://www.wingstop.com", "http://www.anker.com", "http://www.mcdonalds.com", 
        "http://www.bk.com", "http://www.wendys.com",
    ];

    // Max Number of Retries for Each Website Check.
    let max_retries = 3; 

    // For a thread-safe and shared vector among threads in regard to silos that represent states of the websites, I use the `Arc` and `Mutex`.
    let status_vec = Arc::new(Mutex::new(Vec::new()));

    // Initialize a vector to keep handles for every thread.
    let mut handles = vec![];

    // This loop creates a dedicated thread for every input URL, so that they can be run at the same time to check the status of each one's own.
    for url in urls 
    {
        let status_vec = Arc::clone(&status_vec);
        let url = url.to_string();
        // I check the status of a website within a thread using the method `check_website`.
        let handle = thread::spawn(move || {
            let status = check_website(&url, timeout_duration, max_retries);
            let mut status_vec = status_vec.lock().unwrap();
            match status {
                // By using the checki, if it is success then pushes the outcome in the innate vector.
                Ok(website_status) => status_vec.push(website_status),
                // If the check fails, I print an error message.
                Err(e) => println!("Failed to check {}: {}", url, e),
            }
        });
        // I keep the thread handle so I can wait for all threads to finish later.
        handles.push(handle);
    }

    // Like this, I will wait for the joining of all the threads using the `join`.
    for handle in handles 
    {
        handle.join().unwrap();
    }

    // Once all threads have completed their tasks, I lock the shared vector to read the statuses.
    let status_vec = status_vec.lock().unwrap();
    print_stats(&status_vec);
}

// I used this function to retrieve when there is a timeout duration as for HTTP requests.
// We know that a timeout a value with it being a argument on command-line, as for it parses and uses that value.
// If not then it defaults to 5 seconds.
fn get_timeout_duration() -> Duration 
{
    let default_timeout = Duration::from_secs(5);
    match env::args().nth(1) {
        Some(timeout_arg) => timeout_arg.parse().unwrap_or(5),
        None => 5,
    };
    default_timeout
}

// I used this function to perform where there is an actual website statue check.
// I tried to fetch the website when using a timeout to get a maximum number of retrices.
fn check_website(url: &str, timeout: Duration, max_retries: u32) -> Result<website_status::WebsiteStatus, String> 
{
    // I made this to intialize the retry counter.
    let mut retries = 0;

    // I created a loop to continue with a maximum retries as they are reached.
    while retries < max_retries 
    {
        let start_time = Instant::now();

        // Being able to obtain HTTP GET request by using a timeout.
        let response = ureq::get(url).timeout(timeout).call();

        match response {
            // We know if the request succeeds then it able to calculate the elapsed time and return to the statues.
            Ok(response) => {
                let elapsed = start_time.elapsed();
                return Ok(website_status::WebsiteStatus 
                    {
                    url: url.to_string(),
                    status: Ok(response.status()),
                    response_time: elapsed,
                    timestamp: Utc::now(),
                });
            }
            // I set it as if the request fails then it increments the retry counter.
            Err(_) => {
                retries += 1;
                if retries == max_retries 
                {
                    // We know that retries can be exhausted with me being able to return with the failure details.
                    return Err(format!("Max retries ({}) exceeded for {}", max_retries, url));
                }
            }
        }
    }
    // This shouldn't be reached however I was able to return an error if the loops exists unexprected.
    Err("Unexpected loop exit".to_string())
}


// Function to print statistics summary
fn print_stats(status_vec: &Vec<website_status::WebsiteStatus>) {
    let total = status_vec.len();
    let successes: Vec<_> = status_vec.iter().filter(|s| s.status.is_ok()).collect();
    let failures: Vec<_> = status_vec.iter().filter(|s| s.status.is_err()).collect();
    let total_success = successes.len();
    let total_failure = failures.len();

    let mut response_times: Vec<Duration> = successes.iter().map(|s| s.response_time).collect();
    response_times.sort();

    let fastest = response_times.first().cloned().unwrap_or(Duration::from_secs(0));
    let slowest = response_times.last().cloned().unwrap_or(Duration::from_secs(0));
    let average = if !response_times.is_empty() {
        response_times.iter().sum::<Duration>() / response_times.len() as u32
    } else {
        Duration::from_secs(0)
    };

    println!("\nStats Summary:");
    println!("--------------");
    println!("- Total URLs Checked: {}", total);
    println!("- Successful Responses: {}", total_success);
    println!("- Failed Responses: {}", total_failure);
    println!("- Fastest Response Time: {:.2?}", fastest);
    println!("- Slowest Response Time: {:.2?}", slowest);
    println!("- Average Response Time: {:.2?}", average);

    let less_500ms = response_times.iter().filter(|&&t| t < Duration::from_millis(500)).count();
    let between_500ms_1s = response_times.iter().filter(|&&t| t >= Duration::from_millis(500) && t < Duration::from_secs(1)).count();
    let greater_1s = response_times.iter().filter(|&&t| t >= Duration::from_secs(1)).count();

    println!("- Response Time Distribution:");
    println!("  * < 500ms: {}", less_500ms);
    println!("  * 500ms - 1s: {}", between_500ms_1s);
    println!("  * > 1s: {}", greater_1s);
}
