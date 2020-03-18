mod support;

use self::support::{container::MockContainer, make_homo_service, make_redirect_response};
use homochecker_rs::{
    action::request_service,
    domain::{HomoServiceResponse, HomoServiceStatus, Provider},
    repository::Repositories,
    service::Services,
    Container,
};
use std::{net::SocketAddr, sync::Arc, time::Duration};

use http::StatusCode;
use tokio::test as async_test;
use url::Url;

#[async_test]
async fn requests_valid_service() {
    let container = MockContainer::default();
    let source = container.services().homo_request().source();
    let hs = Arc::new(make_homo_service(Provider::Twitter("kb10uy".into())));
    *(source.lock().await) = Box::new(|| {
        (
            make_redirect_response(StatusCode::MOVED_PERMANENTLY, "https://twitter.com/mpyw"),
            Duration::from_secs(1),
        )
    });

    let result = request_service(container.clone(), hs).await.unwrap();
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
