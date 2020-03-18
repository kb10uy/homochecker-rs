mod support;

use self::support::{make_content_response, make_redirect_response};
use homochecker_rs::{
    domain::HomoServiceStatus,
    validation::response::{ResponseHeaderValidator, ResponseHtmlValidator, ValidateResponseExt},
};

use http::StatusCode;
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
    let status = make_redirect_response(StatusCode::MOVED_PERMANENTLY, "https://twitter.com/mpyw")
        .validate::<ResponseHeaderValidator>()
        .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::RedirectResponse),
        "Validating response by header with 301 response"
    );

    let status = make_redirect_response(StatusCode::FOUND, "https://twitter.com/mpyw")
        .validate::<ResponseHeaderValidator>()
        .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::RedirectResponse),
        "Validating response by header with 302 response"
    );

    let status = make_redirect_response(StatusCode::SEE_OTHER, "https://twitter.com/mpyw")
        .validate::<ResponseHeaderValidator>()
        .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::RedirectResponse),
        "Validating response by header with 303 response"
    );

    let status = make_redirect_response(StatusCode::PERMANENT_REDIRECT, "https://twitter.com/mpyw")
        .validate::<ResponseHeaderValidator>()
        .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::RedirectResponse),
        "Validating response by header with 307 response"
    );

    let status = make_redirect_response(StatusCode::TEMPORARY_REDIRECT, "https://twitter.com/mpyw")
        .validate::<ResponseHeaderValidator>()
        .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::RedirectResponse),
        "Validating response by header with 308 response"
    );
}

#[async_test]
async fn checks_response_header_invalid() {
    let status =
        make_redirect_response(StatusCode::MOVED_PERMANENTLY, "https://twitter.com/kb10uy")
            .validate::<ResponseHeaderValidator>()
            .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::Invalid),
        "Validating response by header with 301 wrong redirect"
    );

    let status = make_redirect_response(StatusCode::FOUND, "https://twitter.com/kb10uy")
        .validate::<ResponseHeaderValidator>()
        .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::Invalid),
        "Validating response by header with 302 wrong redirect"
    );

    let status = make_redirect_response(StatusCode::SEE_OTHER, "https://twitter.com/kb10uy")
        .validate::<ResponseHeaderValidator>()
        .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::Invalid),
        "Validating response by header with 303 wrong redirect"
    );

    let status =
        make_redirect_response(StatusCode::PERMANENT_REDIRECT, "https://twitter.com/kb10uy")
            .validate::<ResponseHeaderValidator>()
            .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::Invalid),
        "Validating response by header with 307 wrong redirect"
    );

    let status =
        make_redirect_response(StatusCode::TEMPORARY_REDIRECT, "https://twitter.com/kb10uy")
            .validate::<ResponseHeaderValidator>()
            .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::Invalid),
        "Validating response by header with 308 wrong redirect"
    );
}

#[async_test]
async fn checks_response_header_unknown() {
    let status = make_content_response("text/html", HTML_VALID_REDIRECT)
        .validate::<ResponseHeaderValidator>()
        .await;
    assert_case!(
        status,
        None,
        "Validating response by header with HTML response"
    );
}

#[async_test]
async fn checks_response_content_equiv() {
    let status = make_content_response("text/html", HTML_VALID_REDIRECT)
        .validate::<ResponseHtmlValidator>()
        .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::RedirectContent),
        "Validating response by content with redirecting HTML response"
    );
}

#[async_test]
async fn checks_response_content_contains() {
    let status = make_content_response("text/html", HTML_VALID_CONTENT)
        .validate::<ResponseHtmlValidator>()
        .await;
    assert_case!(
        status,
        Some(HomoServiceStatus::LinkContent),
        "Validating response by content with HTML response which have link"
    );
}

#[async_test]
async fn checks_response_content_err() {
    let status = make_content_response("text/html", HTML_INVALID_CONTENT)
        .validate::<ResponseHtmlValidator>()
        .await;
    assert_case!(
        status,
        None,
        "Validating response by content with invalid HTML response"
    );
}
