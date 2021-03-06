// これがテスト結果に出力されるのを防ぐためのサブモジュール

pub mod container;

use homochecker_rs::domain::{HomoService, HttpResponse, Provider};
use std::collections::HashMap;

use http::StatusCode;
use url::Url;

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

/// Returns specified fixture content.
#[macro_export]
macro_rules! fixture_content {
    ($path:expr) => {{
        use std::{
            fs::File,
            io::{prelude::*, BufReader},
            path::PathBuf,
        };
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(format!("tests/fixture/{}", $path));

        let mut content = vec![];
        let mut file = BufReader::new(File::open(path).unwrap());
        file.read_to_end(&mut content).unwrap();

        content
    }};
}

#[allow(dead_code)]
pub fn make_redirect_response(status: StatusCode, location: &str) -> HttpResponse {
    HttpResponse {
        status,
        headers: {
            let mut h = HashMap::new();
            h.insert("location".into(), location.into());
            h
        },
        remote_address: None,
        body: Default::default(),
    }
}

#[allow(dead_code)]
pub fn make_content_response(content_type: &str, body: &str) -> HttpResponse {
    HttpResponse {
        status: StatusCode::OK,
        headers: {
            let mut h = HashMap::new();
            h.insert("content-type".into(), content_type.into());
            h
        },
        remote_address: None,
        body: body.into(),
    }
}

#[allow(dead_code)]
pub fn make_homo_service(provider: Provider) -> HomoService {
    HomoService {
        provider,
        service_url: Url::parse("https://example.com").unwrap(),
    }
}
