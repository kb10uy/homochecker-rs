mod support;

use self::support::container::MockContainer;
use homochecker_rs::{
    action::fetch_avatar, domain::Provider, repository::Repositories, service::Services, Container,
};
use std::sync::Arc;

use http::response::Builder as ResponseBuilder;
use tokio::test as async_test;
use url::Url;

#[async_test]
async fn fetches_twitter_avatar() {
    let container = {
        let c = MockContainer::default();
        c.services()
            .avatar()
            .for_twitter(|_| {
                let intent_response = fixture_content!("twitter-intent.html");
                ResponseBuilder::new().body(intent_response).unwrap().into()
            })
            .await;
        c
    };

    let provider = Arc::new(Provider::Twitter("kb10uy".into()));
    let result = fetch_avatar(container.clone(), provider.clone()).await;
    let expected = Some(
        Url::parse("https://pbs.twimg.com/profile_images/1217112075673030657/jHdMFr_T_normal.jpg")
            .unwrap(),
    );
    assert_case!(
        result,
        expected,
        "Extracts avatar URL from Twitter user intent HTML"
    );

    let cache = container.repositories().avatar().as_source();
    let locked = cache.lock().await;
    assert_case!(
        locked.get(&*provider),
        expected.as_ref(),
        "Successfully cached in AvatarRepository"
    );
}

#[async_test]
async fn fetches_mastodon_avatar() {
    let container = {
        let c = MockContainer::default();
        c.services()
            .avatar()
            .for_mastodon(|_, _| {
                let user_response = fixture_content!("mastodon-user.json");
                ResponseBuilder::new().body(user_response).unwrap().into()
            })
            .await;
        c
    };

    let provider = Arc::new(Provider::Mastodon {
        screen_name: "kb10uy".into(),
        domain: "mstdn.maud.io".into(),
    });

    let result = fetch_avatar(container.clone(), provider.clone()).await;
    let expected = Some(Url::parse(
        "https://media-mstdn.maud.io/accounts/avatars/000/000/333/original/667d051b2d1e912c.png",
    )
    .unwrap());
    assert_case!(
        result,
        expected,
        "Extracts avatar URL from Twitter user intent HTML"
    );

    let cache = container.repositories().avatar().as_source();
    let locked = cache.lock().await;
    assert_case!(
        locked.get(&*provider),
        expected.as_ref(),
        "Successfully cached in AvatarRepository"
    );
}
