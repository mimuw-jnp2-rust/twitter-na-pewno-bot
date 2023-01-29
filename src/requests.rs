use crate::auth::{get_api_app_context, get_api_user_context};

use std::collections::HashSet;
use time::Date;
use twitter_v2::id::NumericId;
use twitter_v2::query::Exclude::Replies;
use twitter_v2::query::TweetField::{AuthorId, CreatedAt, InReplyToUserId};
use twitter_v2::Tweet;

const MISTAKE: &str = "napewno";
const MINIMUM_NUMBER_OF_RESULTS: usize = 5;
const MAXIMUM_NUMBER_OF_RESULTS: usize = 100;

// Gets id of currently authorized user.
pub async fn get_my_user_id() -> Option<NumericId> {
    let api = get_api_user_context();
    let me = api
        .get_users_me()
        .send()
        .await
        .expect("invalid user")
        .into_data();

    me.map(|user| user.id)
}

// Gets username of currently authorized user.
pub async fn get_my_username() -> Option<String> {
    let api = get_api_user_context();
    let me = api
        .get_users_me()
        .send()
        .await
        .expect("invalid user")
        .into_data();

    me.map(|user| user.username)
}

// Gets username by id.
pub async fn get_username_by_id(id: NumericId) -> Option<String> {
    let api = get_api_user_context();
    let user = api
        .get_user(id)
        .send()
        .await
        .expect("invalid id")
        .into_data();

    user.map(|user| user.username)
}

// Gets name by id.
pub async fn get_name_by_id(id: NumericId) -> Option<String> {
    let api = get_api_user_context();
    let user = api
        .get_user(id)
        .send()
        .await
        .expect("invalid id")
        .into_data();

    user.map(|user| user.name)
}

// Gets id of the latest reply of given user.
pub async fn get_latest_reply_id(user: NumericId) -> Option<NumericId> {
    let api = get_api_app_context();
    let my_tweets = api
        .get_user_tweets(user)
        .tweet_fields([InReplyToUserId])
        .max_results(MINIMUM_NUMBER_OF_RESULTS)
        .send()
        .await
        .expect("invalid user")
        .into_data()
        .unwrap_or_default();

    my_tweets
        .iter()
        .find(|tweet| tweet.in_reply_to_user_id.is_some())
        .map(|tweet| tweet.id)
}

// Gets date of the latest tweet of given user.
pub async fn get_latest_tweet(user: NumericId) -> Option<Tweet> {
    let api = get_api_app_context();
    let my_tweets = api
        .get_user_tweets(user)
        .tweet_fields([CreatedAt])
        .exclude([Replies])
        .max_results(MINIMUM_NUMBER_OF_RESULTS)
        .send()
        .await
        .expect("invalid user")
        .into_data()
        .unwrap_or_default();

    if my_tweets.is_empty() {
        None
    } else {
        Some(my_tweets[0].clone())
    }
}

// Gets tweets with mistake since given tweet.
pub async fn get_tweets_with_mistake(id: Option<NumericId>) -> Vec<Tweet> {
    let api = get_api_app_context();
    let result = if let Some(id) = id {
        api.get_tweets_search_recent(MISTAKE)
            .tweet_fields([AuthorId, CreatedAt])
            .since_id(id)
            .max_results(MAXIMUM_NUMBER_OF_RESULTS)
            .send()
            .await
    } else {
        api.get_tweets_search_recent(MISTAKE)
            .tweet_fields([AuthorId, CreatedAt])
            .max_results(MAXIMUM_NUMBER_OF_RESULTS)
            .send()
            .await
    };

    // Gets no more than last MAXIMUM_NUMBER_OF_RESULTS tweets.
    let mut tweets = result
        .expect("invalid query")
        .into_data()
        .unwrap_or_default();

    // Take oldest tweets first.
    tweets.reverse();
    tweets
}

// Counts all unique users whose tweets included given word on a given day.
pub async fn count_tweets_with_mistake(date: &Date) -> usize {
    let api = get_api_app_context();
    let mut users = HashSet::new();
    let mut size = 1;

    let mut end_date = date
        .next_day()
        .expect("invalid date")
        .midnight()
        .assume_utc();

    while size != 0 {
        let tweets = api
            .get_tweets_search_recent(MISTAKE)
            .tweet_fields([AuthorId, CreatedAt])
            .start_time(date.midnight().assume_utc())
            .end_time(end_date)
            .max_results(MAXIMUM_NUMBER_OF_RESULTS)
            .send()
            .await
            .expect("invalid query")
            .into_data()
            .unwrap_or_default();

        size = tweets
            .iter()
            .map(|tweet| users.insert(tweet.author_id))
            .count();

        if size > 0 {
            end_date = tweets[size - 1].created_at.expect("invalid size");
        }
    }

    users.len()
}

// Posts tweet with given message.
pub async fn post_tweet_with_message(message: String) {
    let api = get_api_user_context();
    api.post_tweet()
        .text(message)
        .send()
        .await
        .expect("invalid message");
}

// Posts reply to provided tweet with given message.
pub async fn post_reply_with_message(id: NumericId, message: String) {
    let api = get_api_user_context();
    api.post_tweet()
        .text(message)
        .in_reply_to_tweet_id(id)
        .send()
        .await
        .expect("invalid message");
}
