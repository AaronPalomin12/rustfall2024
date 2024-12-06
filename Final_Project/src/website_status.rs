// Aaron Palomin
// I'm importing `Duration` from `std::time` to represent the duration with which we receive a response. The current UTC timestamp of checking a website's status is raised using `chrono::Utc`.
use std::time::Duration;
use chrono::Utc;

// Defining struct called WebsiteStatus to encapsulate the website check details.
#[derive(Debug)] // Add Debug to enable printing the struct's fields for debugging purposes and logging.
pub struct WebsiteStatus {
    // URL is a string as it's textual data that is constanly being checkon on upon.
    pub url: String,
    // `status`: This field keeps the HTTP status code of the response or an error message in case the request failed. 
    // It is of type Result which can be of two forms: 
    // - Ok(u16) Contains the HTTP status code... for instance, it could be "200" in case of success and "404" in case data is not found. 
    // - Err(String) Contains an error message in case a request failed.
    pub status: Result<u16, String>,
    // Indicates the time taken for a response to be received from the site: `response_time`. 
    //It is a `Duration' designed to capture elapsed time in an exact form.
    pub response_time: Duration,
    // `timestamp`: The time at which the check on the website happened is recorded in this database field. It uses `chrono::DateTime<Utc>` for precise and timezone-independent timestamps.
    pub timestamp: chrono::DateTime<Utc>,
}
