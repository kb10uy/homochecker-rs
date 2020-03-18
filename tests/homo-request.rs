mod support;

use self::support::{container::MockContainer, make_homo_service, make_redirect_response};
use homochecker_rs::{
    action::{attach_avatar_resolver, request_service},
    domain::{HomoServiceResponse, HomoServiceStatus, Provider},
    service::Services,
    Container,
};
use std::time::Duration;

use http::StatusCode;
use tokio::test as async_test;
use url::Url;

#[async_test]
async fn requests_valid_service() {
    let container = MockContainer::default();
    let source = container.services().homo_request().source();

    let service_url = Url::parse("https://example.org").unwrap();
    *(source.lock().await) = Box::new(|| {
        (
            make_redirect_response(StatusCode::MOVED_PERMANENTLY, "https://twitter.com/mpyw"),
            Duration::from_secs(1),
        )
    });

    let result = request_service(container.clone(), service_url)
        .await
        .unwrap();
    assert_case!(
        result,
        HomoServiceResponse {
            duration: Duration::from_secs(1),
            remote_address: None,
            status: HomoServiceStatus::RedirectResponse,
        },
        "Request for HomoService succeeds"
    );
}

#[async_test]
async fn requests_invalid_service() {
    let container = MockContainer::default();
    let source = container.services().homo_request().source();

    let service_url = Url::parse("https://example.org").unwrap();
    *(source.lock().await) = Box::new(|| {
        (
            make_redirect_response(StatusCode::MOVED_PERMANENTLY, "https://twitter.com/kb10uy"),
            Duration::from_secs(0),
        )
    });

    let result = request_service(container.clone(), service_url)
        .await
        .unwrap();
    assert_case!(
        result,
        HomoServiceResponse {
            duration: Duration::from_secs(0),
            remote_address: None,
            status: HomoServiceStatus::Invalid,
        },
        "Request for HomoService succeeds"
    );
}

#[async_test]
async fn attaches_url_broadcaster() {
    let url = Url::parse("https://kb10uy.org").unwrap();
    let provider = Provider::Mastodon {
        screen_name: "kb10uy".into(),
        domain: "mstdn.maud.io".into(),
    };

    let source = vec![make_homo_service(provider.clone())];
    let (mut sets, txmap) = attach_avatar_resolver(source);
    let rx = &mut sets[0].1;
    let tx = &txmap[&provider];

    tx.send(Some(url.clone())).unwrap();
    let received = rx.recv().await.unwrap();
    assert_case!(received, Some(url), "Equals sent URL and received URL");
}
