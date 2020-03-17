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
    let fixture = fixture_content!("twitter-intent.html");
    let container = MockContainer::default();
    let for_twitter = container.services().avatar().for_twitter();
    let cache = container.repositories().avatar().source();
    *(for_twitter.lock().await) = Box::new(move |_| {
        let fixture = fixture.clone();
        ResponseBuilder::new().body(fixture).unwrap().into()
    });

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

    let locked = cache.lock().await;
    assert_case!(
        locked.get(&*provider),
        expected.as_ref(),
        "Successfully cached in AvatarRepository"
    );
}

#[async_test]
async fn fetches_mastodon_avatar() {
    let fixture = fixture_content!("mastodon-user.json");
    let container = MockContainer::default();
    let for_mastodon = container.services().avatar().for_mastodon();
    let cache = container.repositories().avatar().source();
    *(for_mastodon.lock().await) = Box::new(move |_, _| {
        let fixture = fixture.clone();
        ResponseBuilder::new().body(fixture).unwrap().into()
    });

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

    let locked = cache.lock().await;
    assert_case!(
        locked.get(&*provider),
        expected.as_ref(),
        "Successfully cached in AvatarRepository"
    );
}
