use std::fs;
use std::process::Command;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn extracts_title_from_succesful_res() {
    let server = MockServer::start().await;

    let body = "<html><title>Some Title</title></html>";

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(&server)
        .await;

    let url = server.uri();

    let input_path = "/tmp/input_mock.md";
    let _ = fs::write(input_path, format!("- {url}"));

    Command::new("cargo")
        .args(["run", "--", input_path])
        .output()
        .expect("failed to execute process");

    let output = fs::read_to_string("output.md").unwrap();

    assert_eq!(output.trim(), format!("- [Some Title]({url})"));
}

#[tokio::test]
async fn returns_human_readable_error_for_http_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let url = server.uri();

    let input_path = "/tmp/input_mock_404.md";
    let _ = fs::write(input_path, format!("- {url}"));

    Command::new("cargo")
        .args(["run", "--", input_path])
        .output()
        .expect("failed to execute process");

    let output = fs::read_to_string("output.md").unwrap();

    assert_eq!(output.trim(), format!("- [404 Not Found]({url})"));
}
