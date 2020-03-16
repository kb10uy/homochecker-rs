mod support;

use tokio::test as async_test;

#[async_test]
async fn fetches_twitter_avatar() {
    let intent_response = fixture_content!("twitter-intent.html");
}
