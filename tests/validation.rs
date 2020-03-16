mod support;

use homochecker_rs::{
    domain::HomoServiceStatus,
    validation::response::{ResponseHeaderValidator, ResponseHtmlValidator, ValidateResponseExt},
};

use tokio::test as async_test;

const HTML_VALID_REDIRECT: &str = r#"
    <html>
        <head>
            <meta http-equiv="refresh" content="0;https://twitter.com/mpyw">
            <title>@mpyw</title>
        </head>
        <body>
        </body>
    </html>
"#;
const HTML_VALID_CONTENT: &str = r#"
    <html>
        <head>
            <title>@mpyw</title>
        </head>
        <body>
            https://twitter.com/mpyw
        </body>
    </html>
"#;
const HTML_INVALID_CONTENT: &str = r#"
    <html>
        <head>
            <title>@mpyw</title>
        </head>
        <body>
        </body>
    </html>
"#;

#[async_test]
async fn checks_response_header_ok() {
    let (_mock, url) = support::start_service_redirect(301, "https://twitter.com/mpyw");

    let client = support::create_client();
    let response = client.get(&url).send().await.unwrap();
    let status = response.validate::<ResponseHeaderValidator>().await;

    assert_case!(
        status,
        Some(HomoServiceStatus::RedirectResponse),
        "Validating response by header with redirecting status and header"
    );
}

#[async_test]
async fn checks_response_header_invalid() {
    let (_mock, url) = support::start_service_redirect(301, "https://twitter.com/kb10uy");

    let client = support::create_client();
    let response = client.get(&url).send().await.unwrap();
    let status = response.validate::<ResponseHeaderValidator>().await;

    assert_case!(
        status,
        Some(HomoServiceStatus::Invalid),
        "Validating response by header with redirecting status and invalid header"
    );
}

#[async_test]
async fn checks_response_header_unknown() {
    let (_mock, url) = support::start_service_content("text/html", HTML_VALID_REDIRECT);

    let client = support::create_client();
    let response = client.get(&url).send().await.unwrap();
    let status = response.validate::<ResponseHeaderValidator>().await;

    assert_case!(
        status,
        None,
        "Validating response by header with HTML response"
    );
}

#[async_test]
async fn checks_response_content_equiv() {
    let (_mock, url) = support::start_service_content("text/html", HTML_VALID_REDIRECT);

    let client = support::create_client();
    let response = client.get(&url).send().await.unwrap();
    let status = response.into_validate::<ResponseHtmlValidator>().await;

    assert_case!(
        status,
        Some(HomoServiceStatus::RedirectContent),
        "Validating response by content with redirecting HTML response"
    );
}

#[async_test]
async fn checks_response_content_contains() {
    let (_mock, url) = support::start_service_content("text/html", HTML_VALID_CONTENT);

    let client = support::create_client();
    let response = client.get(&url).send().await.unwrap();
    let status = response.into_validate::<ResponseHtmlValidator>().await;

    assert_case!(
        status,
        Some(HomoServiceStatus::LinkContent),
        "Validating response by content with HTML response which have link"
    );
}

#[async_test]
async fn checks_response_content_err() {
    let (_mock, url) = support::start_service_content("text/html", HTML_INVALID_CONTENT);

    let client = support::create_client();
    let response = client.get(&url).send().await.unwrap();
    let status = response.into_validate::<ResponseHtmlValidator>().await;

    assert_case!(
        status,
        None,
        "Validating response by content with invalid HTML response"
    );
}
