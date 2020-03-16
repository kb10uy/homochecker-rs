// これがテスト結果に出力されるのを防ぐためのサブモジュール

use std::time::Duration;

use mockito::{mock, server_url, Mock};
use reqwest::{redirect::Policy as RedirectPolicy, Client};

/// Pretty-prints assertion case.
#[macro_export]
macro_rules! assert_case {
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        use ansi_term::Colour::{Red, Green, White};
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let assertion_name = format!($($arg)+);
                    println!("{} {}", Red.bold().paint("Failed for assertion:"), White.paint(&assertion_name));
                    println!("  actual  : {}", Red.paint(format!("{:?}", left_val)));
                    println!("  expected: {}", Green.dimmed().paint(format!("{:?}", right_val)));
                    panic!("{}", assertion_name);
                }
            }
        }
    });
}

/// Creates a reqwest `Client` with limited redirect.
pub fn create_client() -> Client {
    Client::builder()
        .redirect(RedirectPolicy::custom(|attempt| {
            // HTTPS ドメインへのリダイレクトだけ飛ぶ
            let prev = &attempt.previous()[0];
            let next = attempt.url();
            if prev.host_str() == next.host_str() {
                attempt.follow()
            } else {
                attempt.stop()
            }
        }))
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

/// Starts a testing service with redirect response.
pub fn start_service_redirect(status: usize, location: &str) -> (Mock, String) {
    let homo = mock("GET", "/")
        .with_status(status)
        .with_header("Location", location)
        .create();

    (homo, server_url())
}

/// Starts a testing service with body response.
pub fn start_service_content(content_type: &str, body: &str) -> (Mock, String) {
    let homo = mock("GET", "/")
        .with_status(200)
        .with_header("Content-Type", content_type)
        .with_body(body)
        .create();

    (homo, server_url())
}
