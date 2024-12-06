// Aaron Palomin
// I'm bringing in all the necessary imports plus crates for a particular function.
// `std::time` employs a variety of tools for the measurement of durations and the recording of time.
// `ureq` is used to make HTTP requests, and `chrono::Utc` helps get the current timestamp.


use std::{time::{Duration, Instant}};
use ureq;
use crate::website_status::WebsiteStatus;
use chrono::Utc;

// The `check_website` function is defined here. It accepts the following arguments: 
// `url`: the URL of the website as a string slice, 
// `timeout_duration`: how long to wait for a response before timing out,
// `retries`: the number of retry attempts allowed if the request fails. 
pub fn check_website(url: &str, timeout_duration: Duration, retries: u8) -> WebsiteStatus 
{
    // The value of retries is assigned to the variable `retries_left`.
    // This variable will keep track of how many retries are left for me in the loop.
    let mut retries_left = retries;

    loop 
    {
        // I note the current time of an incoming request using `Instant::now()` so that I can measure the response time later.
        let start_time = Instant::now();
        // Here, I'm making a GET request to the URL by means of `ureq`.
        // The timeout method set the maximum seconds to wait for the request to process.
        match ureq::get(url)
            .timeout(timeout_duration)
            .call() 
            {
                // In case of successful request, I receive the response and measure the time taken to response.
            Ok(response) => {
                let response_time = start_time.elapsed();
                // I receive a structure in form of Website Status, which contains:
                // The URL,
                // The HTTP Status code, OK
                // The time taken for this response
                // A timestamp indicating the time the check was done.
                return WebsiteStatus 
                {
                    url: url.to_string(),
                    status: Ok(response.status()),
                    response_time,
                    timestamp: Utc::now(),
                };
            }
            // In case a request does not go through, I check if there are remaining retries.
            Err(err) => {

                // In the absence of remaining retries, I return a `WebsiteStatus` to contain:
                //- The URL
                //- The error message stated as `Err`
                //- A default response time as zero 
                //- A timestamp marking the time of failure.
                if retries_left == 0 
                {
                    return WebsiteStatus 
                    {
                        url: url.to_string(),
                        status: Err(format!("Error: {}", err)),
                        response_time: Duration::new(0, 0),
                        timestamp: Utc::now(),
                    };
                }
                // If further attempts are available, the count is decremented and a subsequent attempt is made. 
                retries_left -= 1;
            }
        }
    }
}