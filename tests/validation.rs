mod support;

use homochecker_rs::{
    data::HomoServiceStatus,
    validation::response::{ResponseHeaderValidator, ValidateResponseExt},
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

#[async_test]
async fn checks_response_header_ok() {
    let mocked = support::start_service_redirect(301, "https://twitter.com/mpyw");

    let client = support::create_client();
    let response = client.get(&mocked.1).send().await.unwrap();
    println!("{:?}", response);
    let status = response.validate::<ResponseHeaderValidator>().await;

    assert_case!(
        status,
        Some(HomoServiceStatus::RedirectResponse),
        "Validating response by header with redirecting status and header"
    );
}

#[async_test]
async fn checks_response_header_invalid() {
    let mocked = support::start_service_redirect(301, "https://twitter.com/kb10uy");

    let client = support::create_client();
    let response = client.get(&mocked.1).send().await.unwrap();
    let status = response.validate::<ResponseHeaderValidator>().await;

    assert_case!(
        status,
        Some(HomoServiceStatus::Invalid),
        "Validating response by header with redirecting status and invalid header"
    );
}

#[async_test]
async fn checks_response_header_unknown() {
    let mocked = support::start_service_content("text/html", HTML_VALID_REDIRECT);

    let client = support::create_client();
    let response = client.get(&mocked.1).send().await.unwrap();
    let status = response.validate::<ResponseHeaderValidator>().await;

    assert_case!(
        status,
        None,
        "Validating response by header with HTML response"
    );
}
