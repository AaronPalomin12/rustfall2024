// Aaron Palomin
// Importing the necessary modules for the test.
// `Duration` specifies the amount of time over which the HTTP request should time out.
// `check_website` imported from `http_client` to test its functionality.
// `WebsiteStatus` imports from `website_status` to validate structure and value.
use std::time::Duration; 
use final_project::http_client::check_website;
use final_project::website_status::WebsiteStatus;

// Establishing a unit test for the `check_website` function using a mock server.
#[test]
fn test_check_website_with_mock_server() 
{
    // Set up the mock server URL for testing purposes. Substitute "http://mockserver.com" with a valid mock or test server URL as required.
    let url = "http://mockserver.com";  
    //   Setting the duration of the timeout to 5 seconds for the HTTP request.
    let timeout_duration = Duration::new(5, 0); 
    // Setting up a maximum of 3 retries for the request in case of failure.
    let retries = 3;

    // Invoking the check_website method to see how it could act in the environment of the mocking server.
    let status = check_website(url, timeout_duration, retries);

    // Assertions to check the returned `WebsiteStatus` struct:

    // Confirming that the `url` field of the returned result matches the test input.
    assert_eq!(status.url, url);
    // Inspect whether the field of the `status` indeed contains the value Ok, signifying the successful response.
    assert!(status.status.is_ok());
    // HTTP status code 200 is checked, meaning successful response from server.
    assert_eq!(status.status.unwrap(), 200);
    // Checking whether the response time is defined within the given timeout period, i.e., 5 seconds.
    assert!(status.response_time.as_secs() <= 5); 
}
