// Aaron Palomin
// Importing all required modules and components for the test.
// The 'duration' is meant to define the timeout period for the check of the website.
// Importing the `check_website` function from the `http_client` module which is going to check the functionality of website checking.
// Importing `WebsiteStatus` from the `website_status` module to verify the output of `check_website`.
use std::time::Duration;

use final_project::http_client::check_website;
use final_project::website_status::WebsiteStatus;


// "Building an unit test to check the successful outcome of the function called `check_website`."
#[test]
// Configuring the test using URL valid. Replace with a good test URL http://example.com if needed.
fn test_check_website_success() {
    // Setting up the test along valid URLs. If needed, modify "http://example.com" to a more authentic test URL instead.
    let url = "http://example.com";  // Replace with a test URL
    // Configuring the timeout for checking the website. Here, I configure it as 5 seconds.
    let timeout_duration = Duration::new(5, 0);  // 5 seconds timeout
    // Limits the maximum retry attempts in checking for the website failure .
    let retries = 3;

    // Now calling `check_website` with the parameters provided earlier to see how the function behaves.
    let status = check_website(url, timeout_duration, retries);
    
    // Supposedly creating the `Ok` status for the `result` field denotes that the website checker was a success.
    assert!(status.status.is_ok());
    // The returned 'url' is verified with the input URL for the function providing the correct output.
    assert_eq!(status.url, url);
}
